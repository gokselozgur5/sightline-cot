//! Geodesic cross-track and along-track error to a great circle.

pub use crate::geo_greatcircle::EARTH_RADIUS_M as MEAN_EARTH_RADIUS_M;

/// Cross-track and along-track distances in metres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrossTrackResult {
    pub cross_track_m: f64,
    pub along_track_m: f64,
}

/// Angular distance in radians between two points on a sphere.
pub fn angular_distance_rad(from: [f64; 2], to: [f64; 2]) -> f64 {
    crate::geo_greatcircle::haversine_distance(from[0], from[1], to[0], to[1])
        / crate::geo_greatcircle::EARTH_RADIUS_M
}

/// Initial bearing in radians from one point to another.
pub fn initial_bearing_rad(from: [f64; 2], to: [f64; 2]) -> f64 {
    let d = crate::geo_greatcircle::initial_bearing(from[0], from[1], to[0], to[1]).to_radians();
    if d > std::f64::consts::PI {
        d - std::f64::consts::TAU
    } else {
        d
    }
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

/// Compute both cross-track and along-track error to a great circle.
pub fn cross_track_error(start: [f64; 2], end: [f64; 2], point: [f64; 2]) -> CrossTrackResult {
    CrossTrackResult {
        cross_track_m: cross_track_distance_m(start, end, point),
        along_track_m: along_track_distance_m(start, end, point),
    }
}
