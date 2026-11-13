//! Geodesic cross-track and along-track error to a great circle.

pub const MEAN_EARTH_RADIUS_M: f64 = 6_371_008.8;

/// Cross-track and along-track distances in metres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrossTrackResult {
    pub cross_track_m: f64,
    pub along_track_m: f64,
}

/// Angular distance in radians between two points on a sphere.
pub fn angular_distance_rad(from: [f64; 2], to: [f64; 2]) -> f64 {
    let (lat1, lat2) = (from[0].to_radians(), to[0].to_radians());
    let (d_lat, d_lon) = ((to[0] - from[0]).to_radians(), (to[1] - from[1]).to_radians());
    let a = (d_lat * 0.5).sin().powi(2)
        + lat1.cos() * lat2.cos() * (d_lon * 0.5).sin().powi(2);
    2.0 * a.clamp(0.0, 1.0).sqrt().asin()
}
