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
