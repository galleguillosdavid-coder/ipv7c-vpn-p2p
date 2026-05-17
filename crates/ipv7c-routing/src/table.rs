//! Route table with scoring.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Instant;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub dst_hash: [u8; 4],
    pub next_hop: SocketAddr,
    pub hops: u8,
    pub latency_ms: f64,
    pub trust_score: f64,
    #[serde(skip)]
    pub last_updated: Option<Instant>,
}

impl Route {
    pub fn score(&self) -> f64 {
        let latency_factor = 1.0 / (1.0 + self.latency_ms / 100.0);
        let hop_factor = 1.0 / (1.0 + self.hops as f64);
        self.trust_score * 0.4 + latency_factor * 0.4 + hop_factor * 0.2
    }
}

pub struct RouteTable {
    routes: HashMap<[u8; 4], Vec<Route>>,
}

impl RouteTable {
    pub fn new() -> Self { Self { routes: HashMap::new() } }

    pub fn insert(&mut self, route: Route) {
        let entry = self.routes.entry(route.dst_hash).or_default();
        // Replace if same next_hop, otherwise add
        if let Some(existing) = entry.iter_mut().find(|r| r.next_hop == route.next_hop) {
            *existing = route;
        } else {
            entry.push(route);
        }
    }

    pub fn best_route(&self, dst: &[u8; 4]) -> Option<&Route> {
        self.routes.get(dst)?.iter().max_by(|a, b| a.score().partial_cmp(&b.score()).unwrap_or(std::cmp::Ordering::Equal))
    }

    pub fn remove_stale(&mut self, max_age_secs: u64) {
        for routes in self.routes.values_mut() {
            routes.retain(|r| {
                r.last_updated.map_or(true, |t| t.elapsed().as_secs() < max_age_secs)
            });
        }
        self.routes.retain(|_, v| !v.is_empty());
    }

    pub fn all_destinations(&self) -> Vec<[u8; 4]> {
        self.routes.keys().copied().collect()
    }

    pub fn route_count(&self) -> usize {
        self.routes.values().map(|v| v.len()).sum()
    }
}

impl Default for RouteTable {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_route_by_score() {
        let mut t = RouteTable::new();
        t.insert(Route { dst_hash: [1,2,3,4], next_hop: "1.1.1.1:100".parse().unwrap(), hops: 3, latency_ms: 200.0, trust_score: 0.5, last_updated: None });
        t.insert(Route { dst_hash: [1,2,3,4], next_hop: "2.2.2.2:100".parse().unwrap(), hops: 1, latency_ms: 10.0, trust_score: 0.9, last_updated: None });
        let best = t.best_route(&[1,2,3,4]).unwrap();
        assert_eq!(best.next_hop, "2.2.2.2:100".parse::<SocketAddr>().unwrap());
    }
}
