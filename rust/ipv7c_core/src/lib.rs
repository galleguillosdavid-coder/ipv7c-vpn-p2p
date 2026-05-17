use std::io::{Read, Write};

use base64::{engine::general_purpose, Engine as _};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use serde::{Deserialize, Serialize};

const VERSION: &str = "0.3.0";

fn xor_vec(current: &[u8], previous: &[u8]) -> Vec<u8> {
    let size = current.len().max(previous.len());
    let mut diff = vec![0u8; size];
    for (index, val) in diff.iter_mut().enumerate().take(size) {
        let left = current.get(index).copied().unwrap_or(0);
        let right = previous.get(index).copied().unwrap_or(0);
        *val = left ^ right;
    }
    diff
}

#[derive(Debug, Serialize, Deserialize)]
struct FramePayload {
    encoding: String,
    payload: String,
    size: usize,
    wire_size: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    diff_size: Option<usize>,
}

#[pyfunction]
fn version() -> &'static str {
    VERSION
}

#[pyfunction]
fn xor_bytes<'py>(py: Python<'py>, current: &[u8], previous: &[u8]) -> Py<PyBytes> {
    PyBytes::new(py, &xor_vec(current, previous)).unbind()
}

#[pyfunction]
fn encode_frame_payload_json(payload: &[u8], previous: Option<&[u8]>) -> PyResult<String> {
    if let Some(prev) = previous {
        if payload == prev {
            let frame = FramePayload {
                encoding: "repeat".to_string(),
                payload: String::new(),
                size: payload.len(),
                wire_size: 0,
                diff_size: None,
            };
            return serde_json::to_string(&frame).map_err(|err| pyo3::exceptions::PyValueError::new_err(err.to_string()));
        }

        let diff = xor_vec(payload, prev);
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
        encoder
            .write_all(&diff)
            .map_err(|err| pyo3::exceptions::PyOSError::new_err(err.to_string()))?;
        let compressed = encoder
            .finish()
            .map_err(|err| pyo3::exceptions::PyOSError::new_err(err.to_string()))?;
        if compressed.len() < payload.len() {
            let frame = FramePayload {
                encoding: "xor-zlib".to_string(),
                payload: general_purpose::STANDARD.encode(&compressed),
                size: payload.len(),
                wire_size: compressed.len(),
                diff_size: Some(diff.len()),
            };
            return serde_json::to_string(&frame).map_err(|err| pyo3::exceptions::PyValueError::new_err(err.to_string()));
        }
    }

    let frame = FramePayload {
        encoding: "raw".to_string(),
        payload: general_purpose::STANDARD.encode(payload),
        size: payload.len(),
        wire_size: payload.len(),
        diff_size: None,
    };
    serde_json::to_string(&frame).map_err(|err| pyo3::exceptions::PyValueError::new_err(err.to_string()))
}

#[pyfunction]
fn decode_frame_payload_json<'py>(
    py: Python<'py>,
    frame_json: &str,
    previous: Option<&[u8]>,
) -> PyResult<Option<Py<PyBytes>>> {
    let frame: FramePayload =
        serde_json::from_str(frame_json).map_err(|err| pyo3::exceptions::PyValueError::new_err(err.to_string()))?;
    match frame.encoding.as_str() {
        "repeat" => {
            let prev = match previous {
                Some(prev) => prev,
                None => return Ok(None),
            };
            Ok(Some(PyBytes::new(py, &prev[..frame.size.min(prev.len())]).unbind()))
        }
        "raw" => {
            let raw = general_purpose::STANDARD
                .decode(frame.payload.as_bytes())
                .map_err(|err| pyo3::exceptions::PyValueError::new_err(err.to_string()))?;
            Ok(Some(PyBytes::new(py, &raw).unbind()))
        }
        "xor-zlib" => {
            let prev = match previous {
                Some(prev) => prev,
                None => return Ok(None),
            };
            let raw = general_purpose::STANDARD
                .decode(frame.payload.as_bytes())
                .map_err(|err| pyo3::exceptions::PyValueError::new_err(err.to_string()))?;
            let mut decoder = ZlibDecoder::new(raw.as_slice());
            let mut diff = Vec::new();
            decoder
                .read_to_end(&mut diff)
                .map_err(|err| pyo3::exceptions::PyOSError::new_err(err.to_string()))?;
            let mut decoded = vec![0u8; frame.size];
            for (index, val) in decoded.iter_mut().enumerate().take(frame.size) {
                let before = prev.get(index).copied().unwrap_or(0);
                let delta = diff.get(index).copied().unwrap_or(0);
                *val = before ^ delta;
            }
            Ok(Some(PyBytes::new(py, &decoded).unbind()))
        }
        _ => Ok(None),
    }
}

#[pyfunction]
fn chacha20poly1305_encrypt<'py>(
    py: Python<'py>,
    key: &[u8],
    nonce: &[u8],
    plaintext: &[u8],
    aad: &[u8],
) -> PyResult<Py<PyBytes>> {
    if key.len() != 32 || nonce.len() != 12 {
        return Err(pyo3::exceptions::PyValueError::new_err("key=32 bytes, nonce=12 bytes"));
    }
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let out = cipher
        .encrypt(Nonce::from_slice(nonce), chacha20poly1305::aead::Payload { msg: plaintext, aad })
        .map_err(|_| pyo3::exceptions::PyValueError::new_err("encrypt failed"))?;
    Ok(PyBytes::new(py, &out).unbind())
}

#[pyfunction]
fn chacha20poly1305_decrypt<'py>(
    py: Python<'py>,
    key: &[u8],
    nonce: &[u8],
    ciphertext: &[u8],
    aad: &[u8],
) -> PyResult<Py<PyBytes>> {
    if key.len() != 32 || nonce.len() != 12 {
        return Err(pyo3::exceptions::PyValueError::new_err("key=32 bytes, nonce=12 bytes"));
    }
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let out = cipher
        .decrypt(Nonce::from_slice(nonce), chacha20poly1305::aead::Payload { msg: ciphertext, aad })
        .map_err(|_| pyo3::exceptions::PyValueError::new_err("decrypt failed"))?;
    Ok(PyBytes::new(py, &out).unbind())
}

#[pyfunction]
fn tokio_available() -> bool {
    true
}

#[pymodule]
fn ipv7c_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", VERSION)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(xor_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(encode_frame_payload_json, m)?)?;
    m.add_function(wrap_pyfunction!(decode_frame_payload_json, m)?)?;
    m.add_function(wrap_pyfunction!(chacha20poly1305_encrypt, m)?)?;
    m.add_function(wrap_pyfunction!(chacha20poly1305_decrypt, m)?)?;
    m.add_function(wrap_pyfunction!(tokio_available, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::UdpSocket;

    #[test]
    fn xor_roundtrip() {
        let first = b"aaaaabbbbb";
        let second = b"aaaacbbbbb";
        let diff = xor_vec(second, first);
        let decoded = xor_vec(&diff, first);
        assert_eq!(decoded[..second.len()], second[..]);
    }

    #[test]
    fn codec_json_roundtrip() {
        Python::initialize();
        Python::attach(|py| {
            let first = vec![b'A'; 4096];
            let mut second = first.clone();
            second[10] = b'B';
            let encoded = encode_frame_payload_json(&second, Some(&first)).unwrap();
            let decoded = decode_frame_payload_json(py, &encoded, Some(&first))
                .unwrap()
                .unwrap();
            assert_eq!(decoded.bind(py).as_bytes(), second.as_slice());
        });
    }

    #[test]
    fn chacha_roundtrip() {
        Python::initialize();
        Python::attach(|py| {
            let key = [7u8; 32];
            let nonce = [3u8; 12];
            let aad = b"ipv7c";
            let plaintext = b"secure frame";
            let encrypted = chacha20poly1305_encrypt(py, &key, &nonce, plaintext, aad).unwrap();
            let decrypted = chacha20poly1305_decrypt(py, &key, &nonce, encrypted.bind(py).as_bytes(), aad).unwrap();
            assert_eq!(decrypted.bind(py).as_bytes(), plaintext);
        });
    }

    #[tokio::test]
    async fn tokio_udp_loop_smoke() {
        let server = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let server_addr = server.local_addr().unwrap();
        let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let task = tokio::spawn(async move {
            let mut buf = [0u8; 64];
            let (n, addr) = server.recv_from(&mut buf).await.unwrap();
            server.send_to(&buf[..n], addr).await.unwrap();
        });
        client.send_to(b"tokio-ipv7c", server_addr).await.unwrap();
        let mut buf = [0u8; 64];
        let (n, _) = client.recv_from(&mut buf).await.unwrap();
        task.await.unwrap();
        assert_eq!(&buf[..n], b"tokio-ipv7c");
    }
}
