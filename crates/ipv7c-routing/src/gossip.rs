//! Gossip protocol for route propagation.

use serde::{Serialize, Deserialize};
use crate::table::Route;
use ed25519_dalek::{VerifyingKey, Signature};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GossipMessage {
    pub sender_hash: [u8; 4],
    pub routes: Vec<GossipRoute>,
    pub ttl: u8,
    pub timestamp: u64,
    pub signature: Option<Vec<u8>>, // Ed25519 signature over the serialized message (excluding this field)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GossipRoute {
    pub dst_hash: [u8; 4],
    pub hops: u8,
    pub latency_ms: f64,
    pub trust_score: f64,
}

impl GossipMessage {
    pub fn new(sender_hash: [u8; 4], routes: Vec<Route>) -> Self {
        let gossip_routes = routes.into_iter().map(|r| GossipRoute {
            dst_hash: r.dst_hash, hops: r.hops,
            latency_ms: r.latency_ms, trust_score: r.trust_score,
        }).collect();
        Self {
            sender_hash, routes: gossip_routes, ttl: 5,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
            signature: None,
        }
    }

    pub fn should_propagate(&self) -> bool { self.ttl > 0 }

    pub fn decrement_ttl(mut self) -> Self { self.ttl = self.ttl.saturating_sub(1); self }

    /// Verify signature with a given Ed25519 verifying key bytes.
    /// Returns true if signature is present and valid.
    pub fn verify_signature(&self, vk_bytes: &[u8]) -> bool {
        let sig_bytes = match &self.signature {
            Some(s) => s,
            None => return false,
        };

        // Recreate the serialized payload without signature field
        let mut msg = Vec::new();
        msg.extend_from_slice(&self.sender_hash);
        msg.extend_from_slice(&self.ttl.to_be_bytes());
        msg.extend_from_slice(&self.timestamp.to_be_bytes());
        for r in &self.routes {
            msg.extend_from_slice(&r.dst_hash);
            msg.push(r.hops);
            msg.extend_from_slice(&r.latency_ms.to_be_bytes());
            msg.extend_from_slice(&r.trust_score.to_be_bytes());
        }

        let mut hasher = Sha256::new();
        hasher.update(&msg);
        let digest = hasher.finalize();

        // Convert slices into fixed-size arrays expected by ed25519-dalek
        use std::convert::TryInto;

        let vk_arr: &[u8; 32] = match vk_bytes.try_into() {
            Ok(a) => a,
            Err(_) => return false,
        };

        let sig_arr: &[u8; 64] = match sig_bytes.as_slice().try_into() {
            Ok(a) => a,
            Err(_) => return false,
        };

        let vk = match VerifyingKey::from_bytes(vk_arr) {
            Ok(v) => v,
            Err(_) => return false,
        };

        // Signature::from_bytes returns a Signature directly in this crate version
        let sig = Signature::from_bytes(sig_arr);

        vk.verify_strict(&digest, &sig).is_ok()
    }
}
