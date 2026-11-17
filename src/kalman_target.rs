//! Tactical target velocity vector Kalman prediction filter.

/// Error types for tactical Kalman filtering and state prediction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KalmanTargetError {
    /// Non-finite numerical state or measurement input.
    NonFiniteInput,
    /// Invalid time delta (negative or non-positive when required).
    InvalidDeltaTime,
    /// Matrix inversion failure due to singularity or degeneracy.
    SingularMatrix,
    /// Measurement rejected by residual gating threshold.
    GatedMeasurement { residual: f64, threshold: f64 },
    /// Time delta exceeds configured maximum coast limit.
    CoastLimitExceeded { dt_sec: f64, max_coast_sec: f64 },
}

impl std::fmt::Display for KalmanTargetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteInput => {
                write!(f, "non-finite input encountered in Kalman target filter")
            }
            Self::InvalidDeltaTime => write!(f, "invalid delta time (must be positive and finite)"),
            Self::SingularMatrix => {
                write!(f, "singular or ill-conditioned matrix inversion failure")
            }
            Self::GatedMeasurement {
                residual,
                threshold,
            } => {
                write!(
                    f,
                    "measurement gated out: residual {residual} > gate {threshold}"
                )
            }
            Self::CoastLimitExceeded {
                dt_sec,
                max_coast_sec,
            } => {
                write!(f, "coast limit exceeded: {dt_sec}s > {max_coast_sec}s")
            }
        }
    }
}

impl std::error::Error for KalmanTargetError {}

/// Tuning and validation configuration for tactical target filter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TargetFilterConfig {
    /// Continuous-time acceleration spectral density q (m^2 / s^3).
    pub process_noise_q: f64,
    /// Default position measurement variance (m^2).
    pub default_pos_var: f64,
    /// Default velocity measurement variance (m^2 / s^2).
    pub default_vel_var: f64,
    /// Maximum allowed extrapolation coasting duration (seconds).
    pub max_coast_sec: f64,
    /// Mahalanobis gating threshold (chi-squared gate value).
    pub gate_threshold: f64,
}

impl Default for TargetFilterConfig {
    fn default() -> Self {
        Self {
            process_noise_q: 1.0,
            default_pos_var: 25.0,
            default_vel_var: 4.0,
            max_coast_sec: 60.0,
            gate_threshold: 16.0,
        }
    }
}

impl TargetFilterConfig {
    /// Validate filter configuration values.
    pub fn validate(&self) -> Result<(), KalmanTargetError> {
        let valid = self.process_noise_q.is_finite()
            && self.process_noise_q > 0.0
            && self.default_pos_var.is_finite()
            && self.default_pos_var > 0.0
            && self.default_vel_var.is_finite()
            && self.default_vel_var > 0.0
            && self.max_coast_sec.is_finite()
            && self.max_coast_sec > 0.0
            && self.gate_threshold.is_finite()
            && self.gate_threshold > 0.0;
        if valid {
            Ok(())
        } else {
            Err(KalmanTargetError::NonFiniteInput)
        }
    }
}

/// 3D Kinematic state representing tactical target position and velocity.
#[derive(Debug, Clone, PartialEq)]
pub struct KinematicState {
    /// State vector: [x, y, z, vx, vy, vz] in ENU coordinates.
    pub x: [f64; 6],
    /// 6x6 State error covariance matrix.
    pub cov: [[f64; 6]; 6],
    /// Epoch timestamp in milliseconds.
    pub timestamp_ms: i64,
}

impl KinematicState {
    /// Create a new validated kinematic state.
    pub fn new(
        x: [f64; 6],
        cov: [[f64; 6]; 6],
        timestamp_ms: i64,
    ) -> Result<Self, KalmanTargetError> {
        for v in x.iter() {
            if !v.is_finite() {
                return Err(KalmanTargetError::NonFiniteInput);
            }
        }
