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
