//! Adaptive measurement covariance scaling for Kalman tracking.

/// Error conditions for adaptive covariance scaling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdaptiveCovarianceError {
    NonFiniteInput,
    InvalidConfig,
}

/// Configuration parameters for adaptive measurement covariance scaling.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AdaptiveCovarianceConfig {
    pub gate_threshold: f64,
    pub min_scale: f64,
    pub max_scale: f64,
}

impl Default for AdaptiveCovarianceConfig {
    fn default() -> Self {
        Self {
            gate_threshold: 5.991,
            min_scale: 1.0,
            max_scale: 10.0,
        }
    }
}

impl AdaptiveCovarianceConfig {
    pub fn validate(&self) -> Result<(), AdaptiveCovarianceError> {
        if !self.gate_threshold.is_finite()
            || !self.min_scale.is_finite()
            || !self.max_scale.is_finite()
            || self.gate_threshold <= 0.0
            || self.min_scale < 1.0
            || self.max_scale < self.min_scale
        {
            return Err(AdaptiveCovarianceError::InvalidConfig);
        }
        Ok(())
    }
}

