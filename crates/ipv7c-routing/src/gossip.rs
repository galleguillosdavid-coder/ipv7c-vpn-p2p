//! Gossip protocol for route propagation.

use serde::{Serialize, Deserialize};
use crate::table::Route;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GossipMessage {
    pub sender_hash: [u8; 4],
    pub routes: Vec<GossipRoute>,
    pub ttl: u8,
    pub timestamp: u64,
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
        }
    }

    pub fn should_propagate(&self) -> bool { self.ttl > 0 }

    pub fn decrement_ttl(mut self) -> Self { self.ttl = self.ttl.saturating_sub(1); self }
}
