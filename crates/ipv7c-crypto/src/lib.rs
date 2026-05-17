//! # ipv7c-crypto
//!
//! Post-quantum cryptographic primitives for the IPv7C sovereign mesh VPN.
//!
//! This crate provides:
//! - **ChaCha20-Poly1305** AEAD for session encryption
//! - **X25519** Diffie-Hellman for classical key exchange
//! - **ML-KEM-768** for post-quantum key encapsulation (NIST FIPS 203)
//! - **HKDF-SHA256** for key derivation
//! - **Ed25519** signing (re-exported for identity crate)
//! - Hybrid handshake combining classical + PQC key exchange

pub mod aead;
pub mod kdf;
pub mod kem;
pub mod keys;
pub mod session;

mod error;
pub use error::CryptoError;

/// Re-export Ed25519 for use by the identity crate.
pub use ed25519_dalek;
pub use x25519_dalek;
