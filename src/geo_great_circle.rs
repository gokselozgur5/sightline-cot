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

/// Geodetic waypoint with latitude and longitude in degrees.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Waypoint {
    pub lat: f64,
    pub lon: f64,
}

impl Waypoint {
    /// Create a new waypoint.
    #[inline]
    pub const fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }
}

/// Multi-segment great circle path interpolator.
#[derive(Debug, Clone)]
pub struct GreatCirclePath {
    waypoints: Vec<Waypoint>,
    segment_lengths: Vec<f64>,
    cumulative_lengths: Vec<f64>,
    total_length: f64,
}

/// Standard mean Earth radius in meters (IUGG).
pub const EARTH_RADIUS_M: f64 = 6_371_008.8;

fn validate_coord(wp: &Waypoint) -> Result<(), GreatCircleError> {
    if !wp.lat.is_finite() || !(-90.0..=90.0).contains(&wp.lat) {
        return Err(GreatCircleError::InvalidLatitude);
    }
    if !wp.lon.is_finite() || !(-180.0..=180.0).contains(&wp.lon) {
        return Err(GreatCircleError::InvalidLongitude);
    }
    Ok(())
}

fn to_cartesian(wp: &Waypoint) -> [f64; 3] {
    let (phi, lam) = (wp.lat.to_radians(), wp.lon.to_radians());
    let c = phi.cos();
    [c * lam.cos(), c * lam.sin(), phi.sin()]
}

fn from_cartesian(v: [f64; 3]) -> Waypoint {
    let lat = v[2].atan2((v[0] * v[0] + v[1] * v[1]).sqrt()).to_degrees();
    let lon = (v[1].atan2(v[0]).to_degrees() + 180.0).rem_euclid(360.0) - 180.0;
    Waypoint::new(lat, lon)
}
