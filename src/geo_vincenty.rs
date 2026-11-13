//! Vincenty direct and inverse geodesic algorithms on the WGS84 ellipsoid.

/// Errors from Vincenty geodesic computations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VincentyError {
    InvalidLatitude,
    InvalidLongitude,
    NegativeDistance,
    NonFiniteInput,
    NoConvergence,
}

impl core::fmt::Display for VincentyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLatitude => write!(f, "latitude out of range [-90, 90]"),
            Self::InvalidLongitude => write!(f, "longitude out of range [-180, 180]"),
            Self::NegativeDistance => write!(f, "distance cannot be negative"),
            Self::NonFiniteInput => write!(f, "input values must be finite"),
            Self::NoConvergence => write!(f, "Vincenty formula failed to converge"),
        }
    }
}

impl std::error::Error for VincentyError {}
