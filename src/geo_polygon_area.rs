//! Geodesic polygon area calculator for Cursor-on-Target (CoT) tactical zones.

/// Errors encountered when computing geodesic polygon area.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeodesicAreaError {
    InsufficientVertices,
    NonFiniteCoordinate,
    LatitudeOutOfRange,
    LongitudeOutOfRange,
    DegeneratePolygon,
}

impl core::fmt::Display for GeodesicAreaError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InsufficientVertices => write!(f, "at least 3 vertices required"),
            Self::NonFiniteCoordinate => write!(f, "coordinate is not finite"),
            Self::LatitudeOutOfRange => write!(f, "latitude must be within [-90, 90] degrees"),
            Self::LongitudeOutOfRange => write!(f, "longitude must be within [-180, 180] degrees"),
            Self::DegeneratePolygon => write!(f, "polygon is degenerate with zero or undefined area"),
        }
    }
}

impl std::error::Error for GeodesicAreaError {}
