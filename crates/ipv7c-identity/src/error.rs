//! Identity error types.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("Wallet not found at {0}")]
    WalletNotFound(String),

    #[error("Profile '{0}' already exists")]
    ProfileExists(String),

    #[error("Profile '{0}' not found")]
    ProfileNotFound(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Crypto error: {0}")]
    Crypto(#[from] ipv7c_crypto::CryptoError),

    #[error("Serialization error: {0}")]
    Serialization(String),
}
