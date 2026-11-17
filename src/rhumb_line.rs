//! Rhumb line (loxodrome) navigation solver and waypoint interpolator.

/// Default mean Earth radius in meters (WGS84 spherical approximation).
pub const EARTH_RADIUS_M: f64 = 6_371_008.8;

/// Error types encountered during rhumb line navigation and interpolation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RhumbLineError {
    /// Latitude value outside [-90.0, 90.0] or non-finite.
    InvalidLatitude,
    /// Longitude value outside [-180.0, 180.0] or non-finite.
    InvalidLongitude,
    /// Fewer than two waypoints provided for path.
    InsufficientWaypoints,
    /// Step distance or fraction is invalid, non-positive, or NaN.
    InvalidParameter,
}
