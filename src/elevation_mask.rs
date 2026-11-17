//! Tactical line-of-sight terrain elevation masking angle validator.

/// Errors occurring during elevation mask validation and calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElevationMaskError {
    /// Non-finite coordinate or angle value.
    NonFiniteValue,
    /// Azimuth angle is outside [0.0, 360.0) degrees.
    InvalidAzimuth,
    /// Elevation angle is outside [-90.0, 90.0] degrees.
    InvalidElevation,
    /// Distance or range is non-positive or non-finite.
    InvalidDistance,
    /// Obstacle profile has insufficient points.
    EmptyProfile,
    /// Sector angular span is invalid.
    InvalidSectorSpan,
    /// Profile distance sequence is non-monotonic.
    NonMonotonicProfile,
}

impl core::fmt::Display for ElevationMaskError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFiniteValue => write!(f, "coordinate or angle value is non-finite"),
            Self::InvalidAzimuth => write!(f, "azimuth must be in range [0.0, 360.0) degrees"),
            Self::InvalidElevation => write!(f, "elevation must be in range [-90.0, 90.0] degrees"),
            Self::InvalidDistance => write!(f, "distance must be positive and finite"),
            Self::EmptyProfile => write!(f, "obstacle profile cannot be empty"),
            Self::InvalidSectorSpan => write!(f, "azimuth sector span is invalid"),
            Self::NonMonotonicProfile => write!(f, "obstacle profile distances must strictly increase"),
        }
    }
}

impl std::error::Error for ElevationMaskError {}
