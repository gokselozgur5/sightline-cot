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

/// WGS84 semi-major axis in meters.
pub const WGS84_A_M: f64 = 6_378_137.0;

/// WGS84 flattening.
pub const WGS84_F: f64 = 1.0 / 298.257_223_563;

/// WGS84 first eccentricity squared (e^2 = 2f - f^2).
pub const WGS84_E2: f64 = 0.006_694_379_990_141_316_5;

/// WGS84 authalic sphere radius in meters (R_q = sqrt(A / 4pi)).
pub const WGS84_AUTHALIC_RADIUS_M: f64 = 6_371_007.180_918_474;
