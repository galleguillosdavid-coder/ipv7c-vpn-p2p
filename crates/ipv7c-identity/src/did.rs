//! Decentralized Identifier (DID) derivation from Ed25519 public keys.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::VerifyingKey;
use sha2::{Digest, Sha256};

/// The DID method for IPv7C identities.
const DID_METHOD: &str = "did:ipv7c";

/// A Decentralized Identifier derived from an Ed25519 public key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Did {
    /// Full DID string: `did:ipv7c:<base64url_pubkey>`
    pub uri: String,
    /// 4-byte hash of the DID for use in wire protocol headers.
    pub short_hash: [u8; 4],
}

impl Did {
    /// Derive a DID from an Ed25519 verifying (public) key.
    pub fn from_verifying_key(vk: &VerifyingKey) -> Self {
        let encoded = URL_SAFE_NO_PAD.encode(vk.as_bytes());
        let uri = format!("{DID_METHOD}:{encoded}");

        let mut hasher = Sha256::new();
        hasher.update(uri.as_bytes());
        let full_hash = hasher.finalize();
        let mut short_hash = [0u8; 4];
        short_hash.copy_from_slice(&full_hash[..4]);

        Self { uri, short_hash }
    }

    /// Generate a human-readable alias from the DID hash.
    ///
    /// Format: `AdjectiveAnimal` (e.g., "SwiftCobra", "BraveEagle")
    pub fn alias(&self) -> String {
        const ADJECTIVES: &[&str] = &[
            "Swift", "Brave", "Dark", "Silent", "Iron",
            "Frost", "Storm", "Neon", "Quantum", "Cosmic",
            "Shadow", "Rapid", "Fierce", "Noble", "Hyper",
            "Ultra",
        ];
        const ANIMALS: &[&str] = &[
            "Cobra", "Eagle", "Wolf", "Shark", "Falcon",
            "Tiger", "Lynx", "Raven", "Viper", "Hawk",
            "Puma", "Fox", "Condor", "Dragon", "Phoenix",
            "Jaguar",
        ];

        let adj_idx = self.short_hash[0] as usize % ADJECTIVES.len();
        let ani_idx = self.short_hash[1] as usize % ANIMALS.len();
        format!("{}{}", ADJECTIVES[adj_idx], ANIMALS[ani_idx])
    }
}

impl std::fmt::Display for Did {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ipv7c_crypto::keys::generate_signing_keypair;

    #[test]
    fn did_from_keypair() {
        let sk = generate_signing_keypair();
        let did = Did::from_verifying_key(&sk.verifying_key());

        assert!(did.uri.starts_with("did:ipv7c:"));
        assert_eq!(did.short_hash.len(), 4);
    }

    #[test]
    fn did_deterministic() {
        let sk = generate_signing_keypair();
        let vk = sk.verifying_key();
        let did1 = Did::from_verifying_key(&vk);
        let did2 = Did::from_verifying_key(&vk);
        assert_eq!(did1, did2);
    }

    #[test]
    fn alias_is_readable() {
        let sk = generate_signing_keypair();
        let did = Did::from_verifying_key(&sk.verifying_key());
        let alias = did.alias();
        assert!(!alias.is_empty());
        // Should be two words concatenated
        assert!(alias.len() >= 6);
    }
}
