//! Geocentric to Topocentric (SEZ / AER) coordinate transformer.

/// Error indicating failure during topocentric conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AerError {
    /// Non-finite coordinate or parameter value.
    NonFinite,
    /// Latitude outside the valid range [-90, 90] degrees.
    LatitudeOutOfRange,
    /// Slant range is negative.
    NegativeRange,
    /// Elevation angle outside [-90, 90] degrees.
    ElevationOutOfRange,
}

impl std::error::Error for AerError {}

impl std::fmt::Display for AerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFinite => write!(f, "non-finite coordinate or parameter"),
            Self::LatitudeOutOfRange => write!(f, "latitude must be between -90 and 90 degrees"),
            Self::NegativeRange => write!(f, "slant range cannot be negative"),
            Self::ElevationOutOfRange => write!(f, "elevation must be between -90 and 90 degrees"),
        }
    }
}
