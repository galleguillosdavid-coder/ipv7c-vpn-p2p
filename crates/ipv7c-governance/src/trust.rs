//! Per-peer trust scoring.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustEntry {
    pub did_hash: [u8; 4],
    pub score: f64,
    pub successful_relays: u64,
    pub failed_relays: u64,
    pub uptime_samples: u64,
    pub banned: bool,
}

impl TrustEntry {
    pub fn new(did_hash: [u8; 4]) -> Self {
        Self { did_hash, score: 0.5, successful_relays: 0, failed_relays: 0, uptime_samples: 0, banned: false }
    }

    pub fn record_success(&mut self) {
        self.successful_relays += 1;
        self.recalculate();
    }

    pub fn record_failure(&mut self) {
        self.failed_relays += 1;
        self.recalculate();
    }

    pub fn record_uptime(&mut self) {
        self.uptime_samples += 1;
        self.recalculate();
    }

    fn recalculate(&mut self) {
        let total = self.successful_relays + self.failed_relays;
        if total > 0 {
            let relay_ratio = self.successful_relays as f64 / total as f64;
            let uptime_factor = (self.uptime_samples as f64).min(100.0) / 100.0;
            self.score = relay_ratio * 0.7 + uptime_factor * 0.3;
        }
    }
}

pub struct TrustStore {
    entries: HashMap<[u8; 4], TrustEntry>,
}

impl TrustStore {
    pub fn new() -> Self { Self { entries: HashMap::new() } }

    pub fn get_or_create(&mut self, did_hash: [u8; 4]) -> &mut TrustEntry {
        self.entries.entry(did_hash).or_insert_with(|| TrustEntry::new(did_hash))
    }

    pub fn score(&self, did_hash: &[u8; 4]) -> f64 {
        self.entries.get(did_hash).map_or(0.5, |e| e.score)
    }

    pub fn is_banned(&self, did_hash: &[u8; 4]) -> bool {
        self.entries.get(did_hash).map_or(false, |e| e.banned)
    }

    pub fn ban(&mut self, did_hash: [u8; 4]) {
        let entry = self.get_or_create(did_hash);
        entry.banned = true;
        entry.score = 0.0;
    }

    pub fn decay_all(&mut self, factor: f64) {
        for entry in self.entries.values_mut() {
            if !entry.banned { entry.score *= factor; }
        }
    }
}

impl Default for TrustStore { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trust_increases_with_success() {
        let mut store = TrustStore::new();
        let hash = [1,2,3,4];
        for _ in 0..10 { store.get_or_create(hash).record_success(); }
        assert!(store.score(&hash) > 0.5);
    }

    #[test]
    fn ban_sets_zero() {
        let mut store = TrustStore::new();
        store.ban([1,2,3,4]);
        assert!(store.is_banned(&[1,2,3,4]));
        assert_eq!(store.score(&[1,2,3,4]), 0.0);
    }
}
