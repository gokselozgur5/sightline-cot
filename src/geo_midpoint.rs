//! Geodesic midpoint calculation between two geographic coordinates.

/// Error returned when calculating a geodesic midpoint fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidpointError {
    InvalidLatitude,
    InvalidLongitude,
    AntipodalPoints,
}
