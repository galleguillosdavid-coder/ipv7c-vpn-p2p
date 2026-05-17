//! Encrypted session state with automatic key rotation.

use std::time::Instant;

use crate::aead;
use crate::kdf;
use crate::CryptoError;

/// Maximum bytes before forcing key rotation (1 GB).
const MAX_BYTES_PER_KEY: u64 = 1_073_741_824;

/// Maximum duration before forcing key rotation (10 minutes).
const MAX_DURATION_SECS: u64 = 600;

/// An active encryption session between two peers.
pub struct Session {
    /// The 32-byte symmetric session key.
    key: [u8; 32],
    /// Monotonic sequence number for nonce derivation.
    sequence: u64,
    /// Session identifier for nonce domain separation.
    session_id: [u8; 16],
    /// Total bytes encrypted with this key.
    bytes_encrypted: u64,
    /// When this key was established.
    key_established_at: Instant,
}

impl Session {
    /// Create a new session from a derived key and session ID.
    pub fn new(key: [u8; 32], session_id: [u8; 16]) -> Self {
        Self {
            key,
            sequence: 0,
            session_id,
            bytes_encrypted: 0,
            key_established_at: Instant::now(),
        }
    }

    /// Check if this session's key needs rotation.
    pub fn needs_rotation(&self) -> bool {
        self.bytes_encrypted >= MAX_BYTES_PER_KEY
            || self.key_established_at.elapsed().as_secs() >= MAX_DURATION_SECS
    }

    /// Encrypt a frame payload, advancing the sequence number.
    pub fn encrypt(&mut self, plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nonce = kdf::derive_nonce(&self.session_id, self.sequence);
        let ciphertext = aead::encrypt(&self.key, &nonce, plaintext, aad)?;
        self.sequence += 1;
        self.bytes_encrypted += plaintext.len() as u64;
        Ok(ciphertext)
    }

    /// Decrypt a frame payload at a given sequence number.
    pub fn decrypt(
        &self,
        ciphertext: &[u8],
        sequence: u64,
        aad: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        let nonce = kdf::derive_nonce(&self.session_id, sequence);
        aead::decrypt(&self.key, &nonce, ciphertext, aad)
    }

    /// Current sequence number.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Total bytes encrypted in this session.
    pub fn bytes_encrypted(&self) -> u64 {
        self.bytes_encrypted
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.key.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_session() -> Session {
        let key = [42u8; 32];
        let sid = [1u8; 16];
        Session::new(key, sid)
    }

    #[test]
    fn encrypt_decrypt_session_roundtrip() {
        let mut session = make_session();
        let plaintext = b"hello sovereign mesh";
        let aad = b"frame-header";

        let seq = session.sequence();
        let ciphertext = session.encrypt(plaintext, aad).unwrap();
        let decrypted = session.decrypt(&ciphertext, seq, aad).unwrap();

        assert_eq!(decrypted, plaintext);
        assert_eq!(session.sequence(), 1);
    }

    #[test]
    fn sequence_advances() {
        let mut session = make_session();
        session.encrypt(b"a", b"").unwrap();
        session.encrypt(b"b", b"").unwrap();
        session.encrypt(b"c", b"").unwrap();
        assert_eq!(session.sequence(), 3);
    }

    #[test]
    fn bytes_tracked() {
        let mut session = make_session();
        session.encrypt(&[0u8; 1000], b"").unwrap();
        assert_eq!(session.bytes_encrypted(), 1000);
    }

    #[test]
    fn fresh_session_does_not_need_rotation() {
        let session = make_session();
        assert!(!session.needs_rotation());
    }
}
