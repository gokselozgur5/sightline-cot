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

/// Compute adaptive covariance scale factor from Mahalanobis d2 distance.
pub fn compute_adaptive_scale(
    d2: f64,
    cfg: &AdaptiveCovarianceConfig,
) -> Result<f64, AdaptiveCovarianceError> {
    cfg.validate()?;
    if !d2.is_finite() || d2 < 0.0 {
        return Err(AdaptiveCovarianceError::NonFiniteInput);
    }
    if d2 <= cfg.gate_threshold {
        return Ok(cfg.min_scale);
    }
    let ratio = d2 / cfg.gate_threshold;
    let scale = (cfg.min_scale * ratio).min(cfg.max_scale);
    Ok(scale)
}

/// Scale measurement noise variance R adaptively using Mahalanobis distance.
pub fn scale_measurement_variance(
    r_var: f64,
    d2: f64,
    cfg: &AdaptiveCovarianceConfig,
) -> Result<f64, AdaptiveCovarianceError> {
    if !r_var.is_finite() || r_var <= 0.0 {
        return Err(AdaptiveCovarianceError::NonFiniteInput);
    }
    let scale = compute_adaptive_scale(d2, cfg)?;
    Ok(r_var * scale)
}



