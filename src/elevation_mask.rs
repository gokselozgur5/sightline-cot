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
