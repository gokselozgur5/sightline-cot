//! Line-of-sight elevation and slant-range calculator.

/// Errors occurring during line-of-sight calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeoLosError {
    NonFiniteCoordinate,
    LatitudeOutOfRange,
    CoincidentPoints,
}

impl core::fmt::Display for GeoLosError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFiniteCoordinate => write!(f, "coordinate is non-finite"),
            Self::LatitudeOutOfRange => write!(f, "latitude must be between -90 and 90 degrees"),
            Self::CoincidentPoints => write!(f, "observer and target positions are coincident"),
        }
    }
}

impl std::error::Error for GeoLosError {}
