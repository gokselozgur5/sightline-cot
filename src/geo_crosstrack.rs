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

/// Initial bearing in radians from one point to another.
pub fn initial_bearing_rad(from: [f64; 2], to: [f64; 2]) -> f64 {
    let (lat1, lat2) = (from[0].to_radians(), to[0].to_radians());
    let d_lon = (to[1] - from[1]).to_radians();
    let y = d_lon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * d_lon.cos();
    y.atan2(x)
}

/// Signed cross-track distance in metres from point to the great-circle path start -> end.
///
/// Positive if point is to the right of the path, negative if to the left.
pub fn cross_track_distance_m(start: [f64; 2], end: [f64; 2], point: [f64; 2]) -> f64 {
    let d13 = angular_distance_rad(start, point);
    let theta13 = initial_bearing_rad(start, point);
    let theta12 = initial_bearing_rad(start, end);
    let delta_xt = (d13.sin() * (theta13 - theta12).sin()).asin();
    delta_xt * MEAN_EARTH_RADIUS_M
}

/// Signed along-track distance in metres from path start toward end.
pub fn along_track_distance_m(start: [f64; 2], end: [f64; 2], point: [f64; 2]) -> f64 {
    let d13 = angular_distance_rad(start, point);
    let d_xt = cross_track_distance_m(start, end, point) / MEAN_EARTH_RADIUS_M;
    let cos_d = d13.cos() / d_xt.cos();
    let d_at = cos_d.clamp(-1.0, 1.0).acos();
    let theta13 = initial_bearing_rad(start, point);
    let theta12 = initial_bearing_rad(start, end);
    let sign = (theta13 - theta12).cos().signum();
    sign * d_at * MEAN_EARTH_RADIUS_M
}
