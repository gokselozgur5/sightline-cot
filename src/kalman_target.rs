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

