pub mod p2p;
use pyo3::prelude::*;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{Read, BufReader};

/// Un módulo Rust puro para reemplazar operaciones pesadas en IPv7C.
#[pymodule]
fn ipv7c_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_self_integrity_hash_rust, m)?)?;
    m.add_function(wrap_pyfunction!(init_tokio_runtime, m)?)?;
    m.add_function(wrap_pyfunction!(chacha20poly1305_encrypt, m)?)?;
    m.add_function(wrap_pyfunction!(chacha20poly1305_decrypt, m)?)?;
    m.add_function(wrap_pyfunction!(xor_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(encode_frame_payload_json, m)?)?;
    m.add_function(wrap_pyfunction!(decode_frame_payload_json, m)?)?;
    Ok(())
}

/// Calcula el hash de integridad real usando SHA-256 desde Rust.
#[pyfunction]
#[pyo3(signature = (file_path=None))]
pub fn get_self_integrity_hash_rust(file_path: Option<String>) -> PyResult<String> {
    let path = file_path.unwrap_or_else(|| "ipv7c.py".to_string());
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    
    let result = hasher.finalize();
    let mut hex_str = String::with_capacity(64);
    for byte in result {
        use std::fmt::Write;
        write!(&mut hex_str, "{:02x}", byte).unwrap();
    }
    Ok(hex_str)
}

/// Inicializa el motor asíncrono Tokio para la Fase 5 (Centros de Datos)
#[pyfunction]
fn init_tokio_runtime() -> PyResult<bool> {
    // Scaffold para el runtime asíncrono que manejará +100,000 túneles
    let _rt = tokio::runtime::Runtime::new()?;
    Ok(true)
}

use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    ChaCha20Poly1305, Key, Nonce,
};

#[pyfunction]
fn chacha20poly1305_encrypt(key: &[u8], nonce: &[u8], plaintext: &[u8], aad: &[u8]) -> PyResult<Vec<u8>> {
    let key_arr = Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(key_arr);
    let nonce_arr = Nonce::from_slice(nonce);
    let payload = Payload {
        msg: plaintext,
        aad,
    };
    match cipher.encrypt(nonce_arr, payload) {
        Ok(ciphertext) => Ok(ciphertext),
        Err(_) => Err(pyo3::exceptions::PyValueError::new_err("Encryption failed")),
    }
}

#[pyfunction]
fn chacha20poly1305_decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8], aad: &[u8]) -> PyResult<Vec<u8>> {
    let key_arr = Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(key_arr);
    let nonce_arr = Nonce::from_slice(nonce);
    let payload = Payload {
        msg: ciphertext,
        aad,
    };
    match cipher.decrypt(nonce_arr, payload) {
        Ok(plaintext) => Ok(plaintext),
        Err(_) => Err(pyo3::exceptions::PyValueError::new_err("Decryption failed")),
    }
}

#[pyfunction]
fn xor_bytes(data: &[u8], key: &[u8]) -> PyResult<Vec<u8>> {
    if key.is_empty() {
        return Ok(data.to_vec());
    }
    let mut result = Vec::with_capacity(data.len());
    let mut key_iter = key.iter().cycle();
    for &b in data {
        result.push(b ^ key_iter.next().unwrap());
    }
    Ok(result)
}

use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::io::Write;
use serde_json::json;

#[pyfunction]
fn encode_frame_payload_json(payload: &[u8], previous: Option<&[u8]>) -> PyResult<String> {
    if let Some(prev) = previous {
        if payload == prev {
            let res = json!({
                "encoding": "repeat",
                "payload": "",
                "size": payload.len(),
                "wire_size": 0,
            });
            return Ok(res.to_string());
        }

        let mut diff = Vec::with_capacity(payload.len());
        let mut prev_iter = prev.iter().cycle();
        for &b in payload {
            diff.push(b ^ *prev_iter.next().unwrap_or(&0));
        }

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
        encoder.write_all(&diff).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let compressed = encoder.finish().map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        if compressed.len() < payload.len() {
            let res = json!({
                "encoding": "xor-zlib",
                "payload": BASE64.encode(&compressed),
                "size": payload.len(),
                "diff_size": diff.len(),
                "wire_size": compressed.len(),
            });
            return Ok(res.to_string());
        }
    }

    let res = json!({
        "encoding": "raw",
        "payload": BASE64.encode(payload),
        "size": payload.len(),
        "wire_size": payload.len(),
    });
    Ok(res.to_string())
}

#[pyfunction]
fn decode_frame_payload_json(datos_json: &str, previous: Option<&[u8]>) -> PyResult<Vec<u8>> {
    let datos: serde_json::Value = serde_json::from_str(datos_json)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        
    let encoding = datos.get("encoding").and_then(|v| v.as_str()).unwrap_or("raw");
    let size = datos.get("size").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let payload = datos.get("payload").and_then(|v| v.as_str()).unwrap_or("");
    
    if encoding == "repeat" {
        if let Some(prev) = previous {
            return Ok(prev[..std::cmp::min(size, prev.len())].to_vec());
        }
        return Err(pyo3::exceptions::PyValueError::new_err("No previous payload"));
    }
    
    let raw = BASE64.decode(payload).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    
    if encoding == "raw" {
        return Ok(raw);
    }
    
    if encoding == "xor-zlib" {
        if let Some(prev) = previous {
            let mut decoder = ZlibDecoder::new(&raw[..]);
            let mut diff = Vec::new();
            decoder.read_to_end(&mut diff).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
            
            let mut decoded = vec![0u8; size];
            for i in 0..size {
                let before = *prev.get(i).unwrap_or(&0);
                let delta = *diff.get(i).unwrap_or(&0);
                decoded[i] = before ^ delta;
            }
            return Ok(decoded);
        }
        return Err(pyo3::exceptions::PyValueError::new_err("No previous payload"));
    }
    
    Err(pyo3::exceptions::PyValueError::new_err("Unknown encoding"))
}


#[cfg(target_os = "android")]
pub mod jni_bindings;
