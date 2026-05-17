//! # ipv7c-identity
//!
//! Decentralized identity management with multi-profile support.
//!
//! - Ed25519 keypair generation → DID derivation
//! - Encrypted wallet store (SQLite + ChaCha20)
//! - Multi-profile: separate keypair + config per profile
//! - Human-readable alias auto-generation

pub mod did;
pub mod profile;
pub mod wallet;

mod error;
pub use error::IdentityError;
