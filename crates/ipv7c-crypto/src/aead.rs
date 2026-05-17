//! ChaCha20-Poly1305 AEAD encryption and decryption.
//!
//! All data frames on the wire are encrypted with this cipher.
//! Key: 32 bytes, Nonce: 12 bytes, Tag: 16 bytes appended to ciphertext.

use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};

use crate::CryptoError;

/// AEAD key size in bytes.
pub const KEY_SIZE: usize = 32;

/// AEAD nonce size in bytes.
pub const NONCE_SIZE: usize = 12;

/// AEAD authentication tag size in bytes.
pub const TAG_SIZE: usize = 16;

/// Encrypt plaintext with ChaCha20-Poly1305 AEAD.
///
/// Returns ciphertext with the 16-byte authentication tag appended.
pub fn encrypt(
    key: &[u8; KEY_SIZE],
    nonce: &[u8; NONCE_SIZE],
    plaintext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let payload = Payload {
        msg: plaintext,
        aad,
    };
    cipher
        .encrypt(Nonce::from_slice(nonce), payload)
        .map_err(|_| CryptoError::EncryptionFailed)
}

/// Decrypt ciphertext with ChaCha20-Poly1305 AEAD.
///
/// The ciphertext must include the 16-byte authentication tag.
pub fn decrypt(
    key: &[u8; KEY_SIZE],
    nonce: &[u8; NONCE_SIZE],
    ciphertext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let payload = Payload {
        msg: ciphertext,
        aad,
    };
    cipher
        .decrypt(Nonce::from_slice(nonce), payload)
        .map_err(|_| CryptoError::DecryptionFailed)
}

/// Generate a cryptographically secure random nonce.
pub fn generate_nonce() -> [u8; NONCE_SIZE] {
    let mut nonce = [0u8; NONCE_SIZE];
    rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut nonce);
    nonce
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let key = [7u8; KEY_SIZE];
        let nonce = generate_nonce();
        let plaintext = b"sovereign mesh frame data";
        let aad = b"ipv7c-v1";

        let ciphertext = encrypt(&key, &nonce, plaintext, aad).unwrap();
        assert_ne!(ciphertext, plaintext);

        let decrypted = decrypt(&key, &nonce, &ciphertext, aad).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn decrypt_wrong_key_fails() {
        let key = [7u8; KEY_SIZE];
        let wrong_key = [9u8; KEY_SIZE];
        let nonce = generate_nonce();
        let plaintext = b"test data";
        let aad = b"";

        let ciphertext = encrypt(&key, &nonce, plaintext, aad).unwrap();
        let result = decrypt(&wrong_key, &nonce, &ciphertext, aad);
        assert!(result.is_err());
    }

    #[test]
    fn decrypt_tampered_ciphertext_fails() {
        let key = [7u8; KEY_SIZE];
        let nonce = generate_nonce();
        let plaintext = b"test data";
        let aad = b"";

        let mut ciphertext = encrypt(&key, &nonce, plaintext, aad).unwrap();
        if let Some(byte) = ciphertext.get_mut(0) {
            *byte ^= 0xFF;
        }
        let result = decrypt(&key, &nonce, &ciphertext, aad);
        assert!(result.is_err());
    }

    #[test]
    fn empty_plaintext_works() {
        let key = [1u8; KEY_SIZE];
        let nonce = generate_nonce();
        let ciphertext = encrypt(&key, &nonce, b"", b"").unwrap();
        let decrypted = decrypt(&key, &nonce, &ciphertext, b"").unwrap();
        assert!(decrypted.is_empty());
    }
}
