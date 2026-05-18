//! Metrics utilities (EWMA)

#[derive(Debug, Clone)]
pub struct Ewma {
    alpha: f64,
    value: Option<f64>,
}

impl Ewma {
    pub fn new(alpha: f64) -> Self { Self { alpha, value: None } }

    pub fn update(&mut self, sample: f64) {
        self.value = Some(match self.value {
            None => sample,
            Some(v) => v + self.alpha * (sample - v),
        });
    }

    pub fn value(&self) -> f64 { self.value.unwrap_or(0.0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ewma_basic() {
        let mut e = Ewma::new(0.5);
        e.update(10.0);
        assert!((e.value() - 10.0).abs() < 1e-6);
        e.update(20.0);
        assert!(e.value() > 10.0 && e.value() < 20.0);
    }
}
