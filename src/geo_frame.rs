//! Geodetic LocalFrame with cached reference trigonometry.

/// Error indicating failure during LocalFrame creation or projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeoFrameError {
    /// Non-finite coordinate or parameter value encountered.
    NonFinite,
    /// Latitude outside the valid range [-90, 90] degrees.
    LatitudeOutOfRange,
}

impl std::fmt::Display for GeoFrameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFinite => write!(f, "non-finite coordinate or parameter"),
            Self::LatitudeOutOfRange => write!(f, "latitude must be between -90 and 90 degrees"),
        }
    }
}

impl std::error::Error for GeoFrameError {}
