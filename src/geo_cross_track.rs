use crate::geo_greatcircle::{haversine_distance, initial_bearing, EARTH_RADIUS_M};

/// Signed cross-track distance in meters from a point to the great circle
/// through start and end. Positive means right of the path direction.
pub fn cross_track_distance(start: (f64, f64), end: (f64, f64), point: (f64, f64)) -> f64 {
    let d13 = haversine_distance(start.0, start.1, point.0, point.1) / EARTH_RADIUS_M;
    let t13 = initial_bearing(start.0, start.1, point.0, point.1).to_radians();
    let t12 = initial_bearing(start.0, start.1, end.0, end.1).to_radians();
    (d13.sin() * (t13 - t12).sin()).asin() * EARTH_RADIUS_M
}
