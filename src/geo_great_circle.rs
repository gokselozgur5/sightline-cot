//! Great Circle waypoint path interpolator.

/// Errors encountered during path construction or interpolation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GreatCircleError {
    /// Latitude value outside [-90.0, 90.0] or non-finite.
    InvalidLatitude,
    /// Longitude value outside [-180.0, 180.0] or non-finite.
    InvalidLongitude,
    /// Fewer than two waypoints provided.
    InsufficientWaypoints,
    /// Adjacent waypoints are antipodal (180 degrees apart).
    AntipodalWaypoints,
    /// Distance parameter is invalid or out of range.
    InvalidDistance,
    /// Step or count parameter is zero or non-positive.
    InvalidParameter,
}
