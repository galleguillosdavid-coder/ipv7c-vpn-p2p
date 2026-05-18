//! ScoreCalculator and profiles for route scoring

/// Pre-defined scoring profiles
#[derive(Debug, Clone, Copy)]
pub enum ScoreProfile {
    Speed,
    Security,
    Stability,
    Balanced,
}

/// Trait describing measurable route metrics. Implemented by `table::Route`.
pub trait Scoreable {
    fn latency_ms(&self) -> f64;
    fn packet_loss(&self) -> f64;
    fn congestion(&self) -> f64;
    fn trust_score(&self) -> f64;
    fn hops(&self) -> u8;
}

pub struct ScoreCalculator {}

impl ScoreCalculator {
    pub fn new() -> Self { Self {} }

    pub fn weights(profile: ScoreProfile) -> (f64,f64,f64,f64,f64) {
        match profile {
            ScoreProfile::Speed => (0.50, 0.20, 0.10, 0.10, 0.10),
            ScoreProfile::Security => (0.15, 0.20, 0.10, 0.45, 0.10),
            ScoreProfile::Stability => (0.20, 0.30, 0.25, 0.15, 0.10),
            ScoreProfile::Balanced => (0.30, 0.25, 0.15, 0.20, 0.10),
        }
    }

    pub fn score<S: Scoreable>(&self, s: &S, profile: ScoreProfile) -> f64 {
        // Normalize metrics into 0..1
        let latency = 1.0 / (1.0 + s.latency_ms() / 100.0);
        let loss = 1.0 - s.packet_loss().clamp(0.0, 1.0);
        let congestion = 1.0 - s.congestion().clamp(0.0, 1.0);
        let trust = s.trust_score().clamp(0.0, 1.0);
        let hop_factor = 1.0 / (1.0 + s.hops() as f64);

        let (w_l, w_p, w_c, w_t, w_h) = Self::weights(profile);

        // Higher is better
        w_l * latency + w_p * loss + w_c * congestion + w_t * trust + w_h * hop_factor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyRoute { latency: f64, loss: f64, cong: f64, trust: f64, hops: u8 }

    impl Scoreable for DummyRoute {
        fn latency_ms(&self) -> f64 { self.latency }
        fn packet_loss(&self) -> f64 { self.loss }
        fn congestion(&self) -> f64 { self.cong }
        fn trust_score(&self) -> f64 { self.trust }
        fn hops(&self) -> u8 { self.hops }
    }

    #[test]
    fn score_prefers_trusty_low_latency() {
        let calc = ScoreCalculator::new();
        let r1 = DummyRoute { latency: 30.0, loss: 0.02, cong: 0.1, trust: 0.5, hops: 3 };
        let r2 = DummyRoute { latency: 60.0, loss: 0.005, cong: 0.05, trust: 0.9, hops: 2 };
        let s1 = calc.score(&r1, ScoreProfile::Balanced);
        let s2 = calc.score(&r2, ScoreProfile::Balanced);
        assert!(s2 > s1);
    }
}
