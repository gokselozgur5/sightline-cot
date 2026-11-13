//! Geodetic polyline length and fixed-spacing resampling.

pub const WGS84_EARTH_RADIUS: f64 = 6_371_008.8;

/// Haversine distance in metres on a sphere of given radius.
pub fn haversine_distance_radius(p1: [f64; 2], p2: [f64; 2], radius: f64) -> f64 {
    let (lat1, lat2) = (p1[0].to_radians(), p2[0].to_radians());
    let (dlat, dlon) = ((p2[0] - p1[0]).to_radians(), (p2[1] - p1[1]).to_radians());
    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    2.0 * radius * a.clamp(0.0, 1.0).sqrt().asin()
}

/// Haversine distance in metres using WGS84 mean earth radius.
pub fn haversine_distance(p1: [f64; 2], p2: [f64; 2]) -> f64 {
    haversine_distance_radius(p1, p2, WGS84_EARTH_RADIUS)
}
