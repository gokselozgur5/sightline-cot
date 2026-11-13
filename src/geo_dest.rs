//! Geodesic destination point and distance computations.

/// Errors that can occur during geodesic destination calculation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeoDestError {
    InvalidLatitude,
    InvalidLongitude,
    NegativeRange,
    NonFiniteInput,
}

impl core::fmt::Display for GeoDestError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLatitude => write!(f, "latitude out of range [-90, 90]"),
            Self::InvalidLongitude => write!(f, "longitude out of range [-180, 180]"),
            Self::NegativeRange => write!(f, "range cannot be negative"),
            Self::NonFiniteInput => write!(f, "input values must be finite"),
        }
    }
}

impl std::error::Error for GeoDestError {}
