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

/// Geographic waypoint in decimal degrees.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RhumbWaypoint {
    pub lat: f64,
    pub lon: f64,
}

impl RhumbWaypoint {
    /// Create a new waypoint with latitude and longitude.
    pub const fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }

    /// Validate latitude in [-90, 90] and longitude in [-180, 180].
    pub fn validate(&self) -> Result<(), RhumbLineError> {
        if !self.lat.is_finite() || !(-90.0..=90.0).contains(&self.lat) {
            return Err(RhumbLineError::InvalidLatitude);
        }
        if !self.lon.is_finite() || !(-180.0..=180.0).contains(&self.lon) {
            return Err(RhumbLineError::InvalidLongitude);
        }
        Ok(())
    }
}
