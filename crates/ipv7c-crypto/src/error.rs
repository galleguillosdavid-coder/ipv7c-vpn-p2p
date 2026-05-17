//! Cryptographic error types.

use thiserror::Error;

/// All cryptographic errors in IPv7C.
#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("AEAD encryption failed")]
    EncryptionFailed,

    #[error("AEAD decryption failed: invalid ciphertext or authentication tag")]
    DecryptionFailed,

    #[error("Invalid key length: expected {expected}, got {got}")]
    InvalidKeyLength { expected: usize, got: usize },

    #[error("Invalid nonce length: expected {expected}, got {got}")]
    InvalidNonceLength { expected: usize, got: usize },

    #[error("Key derivation failed: {0}")]
    KdfFailed(String),

    #[error("KEM encapsulation failed")]
    KemEncapsulationFailed,

    #[error("KEM decapsulation failed")]
    KemDecapsulationFailed,

    #[error("Signature verification failed")]
    SignatureInvalid,
}
