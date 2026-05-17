//! Multi-profile management.
//!
//! Each profile has its own keypair, DID, routing preferences, and alias.

use serde::{Deserialize, Serialize};

use crate::did::Did;

/// A named network profile with its own identity and preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Profile name (e.g., "work", "home", "gaming", "anon").
    pub name: String,
    /// Ed25519 signing key bytes (encrypted at rest in wallet).
    #[serde(with = "serde_bytes_array")]
    pub signing_key: [u8; 32],
    /// Routing preference for this profile.
    pub routing: RoutingPreference,
    /// Maximum relay hops allowed.
    pub max_hops: u8,
    /// Whether to allow direct connections or force relay.
    pub direct_allowed: bool,
    /// Creation timestamp (UNIX seconds).
    pub created_at: u64,
}

/// Routing strategy for a profile.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RoutingPreference {
    /// Prefer the lowest latency path.
    LowestLatency,
    /// Prefer the most reliable path (highest trust).
    MostReliable,
    /// Prefer maximum privacy (more hops, no direct).
    MaxPrivacy,
}

impl Default for RoutingPreference {
    fn default() -> Self {
        Self::LowestLatency
    }
}

impl Profile {
    /// Create a new profile with a freshly generated keypair.
    pub fn new(name: &str) -> Self {
        let sk = ipv7c_crypto::keys::generate_signing_keypair();
        Self {
            name: name.to_string(),
            signing_key: sk.to_bytes(),
            routing: RoutingPreference::default(),
            max_hops: 3,
            direct_allowed: true,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Get the DID for this profile.
    pub fn did(&self) -> Did {
        let sk = ipv7c_crypto::keys::import_signing_key(&self.signing_key);
        Did::from_verifying_key(&sk.verifying_key())
    }

    /// Get the human-readable alias for this profile.
    pub fn alias(&self) -> String {
        self.did().alias()
    }
}

/// Serde helper for fixed-size byte arrays.
mod serde_bytes_array {
    use serde::{Deserialize, Deserializer, Serializer};
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;

    pub fn serialize<S>(bytes: &[u8; 32], s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&STANDARD.encode(bytes))
    }

    pub fn deserialize<'de, D>(d: D) -> Result<[u8; 32], D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded = String::deserialize(d)?;
        let decoded = STANDARD.decode(encoded).map_err(serde::de::Error::custom)?;
        decoded
            .try_into()
            .map_err(|_| serde::de::Error::custom("expected 32 bytes"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_profile() {
        let p = Profile::new("work");
        assert_eq!(p.name, "work");
        assert!(p.direct_allowed);
        assert_eq!(p.routing, RoutingPreference::LowestLatency);
    }

    #[test]
    fn profile_did_is_stable() {
        let p = Profile::new("test");
        let did1 = p.did();
        let did2 = p.did();
        assert_eq!(did1, did2);
    }

    #[test]
    fn profile_serialize_roundtrip() {
        let p = Profile::new("gaming");
        let json = serde_json::to_string(&p).unwrap();
        let restored: Profile = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.name, "gaming");
        assert_eq!(restored.signing_key, p.signing_key);
    }
}
