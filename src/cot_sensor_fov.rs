//! CoT sensor field-of-view (FOV) frustum and cone calculator.

/// Errors occurring during sensor FOV calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorFovError {
    NonFiniteParameter,
    InvalidFovAngle,
    InvalidRange,
    InvalidElevation,
    RayDoesNotIntersectGround,
}

impl core::fmt::Display for SensorFovError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFiniteParameter => write!(f, "parameter is non-finite"),
            Self::InvalidFovAngle => write!(f, "FOV angle must be in (0.0, 180.0) degrees"),
            Self::InvalidRange => write!(f, "range must be strictly positive"),
            Self::InvalidElevation => write!(f, "elevation must be in [-90.0, 90.0] degrees"),
            Self::RayDoesNotIntersectGround => write!(f, "sensor ray does not intersect ground"),
        }
    }
}

impl std::error::Error for SensorFovError {}
