//! AI-Driven routing intelligence for Sovereign Mesh.

use std::collections::HashMap;
use std::net::SocketAddr;
use crate::table::{RouteTable, Route};
use std::time::{SystemTime, UNIX_EPOCH};

/// Predictive model for learning traffic patterns
pub struct PredictiveModel {
    // Record hours of activity per IP
    activity_log: HashMap<SocketAddr, Vec<u64>>,
}

impl PredictiveModel {
    pub fn new() -> Self {
        Self {
            activity_log: HashMap::new(),
        }
    }

    pub fn record_activity(&mut self, addr: SocketAddr) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.activity_log.entry(addr).or_insert_with(Vec::new).push(now);
    }

    pub fn should_pre_connect(&self, addr: &SocketAddr) -> bool {
        // Mock logic: if we have more than 5 activity records, we should pre-connect
        self.activity_log.get(addr).map(|logs| logs.len() > 5).unwrap_or(false)
    }
}

/// Anomaly detection and autonomous route adjustment.
pub struct RoutingIntelligence {
    quarantine_list: HashMap<SocketAddr, f64>, // node to anomaly score
    anomaly_threshold: f64,
    pub predictive_model: PredictiveModel,
}

impl RoutingIntelligence {
    pub fn new() -> Self {
        Self {
            quarantine_list: HashMap::new(),
            anomaly_threshold: 0.85,
            predictive_model: PredictiveModel::new(),
        }
    }

    /// Evaluates telemetry and adjusts trust scores.
    /// If latency is too high or hops fluctuate wildly, decreases trust.
    pub fn evaluate_route(&mut self, route: &mut Route) {
        let mut anomaly_score = 0.0;
        
        if route.latency_ms > 500.0 {
            anomaly_score += 0.3;
        } else if route.latency_ms > 200.0 {
            anomaly_score += 0.1;
        }

        if route.hops > 10 {
            anomaly_score += 0.4;
        }

        // Apply penalty
        if anomaly_score > 0.0 {
            route.trust_score *= 1.0 - (anomaly_score * 0.5); // Decay trust
        } else {
            route.trust_score = (route.trust_score + 0.1).min(1.0); // Reward stability
            self.predictive_model.record_activity(route.next_hop);
        }

        if anomaly_score >= self.anomaly_threshold {
            let score = self.quarantine_list.entry(route.next_hop).or_insert(0.0);
            *score += anomaly_score;
        }
    }

    /// Iterates over all routes in a table and applies AI heuristics
    pub fn auto_repair(&mut self, table: &mut RouteTable) {
        let mut to_remove = Vec::new();
        
        for (dst, routes) in table.routes_mut() {
            for route in routes.iter_mut() {
                self.evaluate_route(route);
                
                // If quarantined
                if let Some(&score) = self.quarantine_list.get(&route.next_hop) {
                    if score > 2.0 {
                        // Mark for removal
                        to_remove.push((*dst, route.next_hop));
                    }
                }
            }
        }
        
        // Quarantine logic: remove bad routes completely
        for (dst, next_hop) in to_remove {
            table.remove_route(&dst, &next_hop);
        }
    }
}
