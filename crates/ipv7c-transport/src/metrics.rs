//! Per-peer transport metrics.

use std::time::Instant;

#[derive(Debug, Clone)]
pub struct PeerMetrics {
    pub packets_sent: u64,
    pub packets_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub rtt_samples: Vec<f64>,
    pub last_activity: Instant,
}

impl PeerMetrics {
    pub fn new() -> Self {
        Self {
            packets_sent: 0, packets_received: 0,
            bytes_sent: 0, bytes_received: 0,
            rtt_samples: Vec::new(),
            last_activity: Instant::now(),
        }
    }

    pub fn record_send(&mut self, bytes: usize) {
        self.packets_sent += 1;
        self.bytes_sent += bytes as u64;
        self.last_activity = Instant::now();
    }

    pub fn record_recv(&mut self, bytes: usize) {
        self.packets_received += 1;
        self.bytes_received += bytes as u64;
        self.last_activity = Instant::now();
    }

    pub fn record_rtt(&mut self, rtt_ms: f64) {
        self.rtt_samples.push(rtt_ms);
        if self.rtt_samples.len() > 100 { self.rtt_samples.remove(0); }
    }

    pub fn avg_rtt(&self) -> Option<f64> {
        if self.rtt_samples.is_empty() { return None; }
        Some(self.rtt_samples.iter().sum::<f64>() / self.rtt_samples.len() as f64)
    }

    pub fn packet_loss_ratio(&self) -> f64 {
        if self.packets_sent == 0 { return 0.0; }
        let expected = self.packets_sent;
        let received = self.packets_received;
        if received >= expected { 0.0 }
        else { (expected - received) as f64 / expected as f64 }
    }
}

impl Default for PeerMetrics {
    fn default() -> Self { Self::new() }
}
