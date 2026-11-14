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
