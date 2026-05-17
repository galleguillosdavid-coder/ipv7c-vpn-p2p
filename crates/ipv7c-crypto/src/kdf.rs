//! Key derivation using HKDF-SHA256.
//!
//! Used to derive session keys from shared secrets produced by
//! the hybrid X25519 + ML-KEM key exchange.

use hkdf::Hkdf;
use sha2::Sha256;

use crate::CryptoError;

/// Derive a fixed-length key from input keying material using HKDF-SHA256.
///
/// - `ikm`: Input keying material (shared secret from key exchange)
/// - `salt`: Optional salt (use peer DIDs concatenated for binding)
/// - `info`: Context info (e.g., "ipv7c-session-key-v1")
/// - `output`: Buffer to write derived key material into
pub fn derive_key(
    ikm: &[u8],
    salt: Option<&[u8]>,
    info: &[u8],
    output: &mut [u8],
) -> Result<(), CryptoError> {
    let hk = Hkdf::<Sha256>::new(salt, ikm);
    hk.expand(info, output)
        .map_err(|e| CryptoError::KdfFailed(e.to_string()))
}

/// Derive a 32-byte session key from a hybrid shared secret.
///
/// Combines the classical (X25519) and post-quantum (ML-KEM) shared secrets
/// by concatenating them before HKDF extraction.
pub fn derive_session_key(
    classical_secret: &[u8],
    pqc_secret: &[u8],
    salt: &[u8],
) -> Result<[u8; 32], CryptoError> {
    let mut combined = Vec::with_capacity(classical_secret.len() + pqc_secret.len());
    combined.extend_from_slice(classical_secret);
    combined.extend_from_slice(pqc_secret);

    let mut session_key = [0u8; 32];
    derive_key(&combined, Some(salt), b"ipv7c-session-key-v1", &mut session_key)?;
    Ok(session_key)
}

/// Derive a 12-byte nonce from a sequence number and session context.
pub fn derive_nonce(session_id: &[u8], sequence: u64) -> [u8; 12] {
    let mut material = Vec::with_capacity(session_id.len() + 8);
    material.extend_from_slice(session_id);
    material.extend_from_slice(&sequence.to_le_bytes());

    let hk = Hkdf::<Sha256>::new(None, &material);
    let mut nonce = [0u8; 12];
    // This cannot fail for output length ≤ 255 * hash_len
    hk.expand(b"ipv7c-nonce-v1", &mut nonce).unwrap();
    nonce
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_key_deterministic() {
        let mut out1 = [0u8; 32];
        let mut out2 = [0u8; 32];
        derive_key(b"secret", Some(b"salt"), b"info", &mut out1).unwrap();
        derive_key(b"secret", Some(b"salt"), b"info", &mut out2).unwrap();
        assert_eq!(out1, out2);
    }

    #[test]
    fn different_inputs_different_keys() {
        let mut out1 = [0u8; 32];
        let mut out2 = [0u8; 32];
        derive_key(b"secret-a", Some(b"salt"), b"info", &mut out1).unwrap();
        derive_key(b"secret-b", Some(b"salt"), b"info", &mut out2).unwrap();
        assert_ne!(out1, out2);
    }

    #[test]
    fn session_key_combines_both_secrets() {
        let classical = [1u8; 32];
        let pqc = [2u8; 32];
        let salt = b"did:ipv7c:alice+did:ipv7c:bob";

        let key = derive_session_key(&classical, &pqc, salt).unwrap();
        assert_ne!(key, [0u8; 32]);
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn nonce_varies_with_sequence() {
        let session_id = b"test-session";
        let n1 = derive_nonce(session_id, 0);
        let n2 = derive_nonce(session_id, 1);
        assert_ne!(n1, n2);
    }
}
