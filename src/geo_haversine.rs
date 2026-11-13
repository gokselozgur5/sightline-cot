//! Haversine formula for great-circle distance and initial bearing on a sphere.

pub const MEAN_EARTH_RADIUS_M: f64 = 6_371_008.8;

/// Half-versine of an angle in radians: sin^2(theta / 2).
pub fn hav(theta_rad: f64) -> f64 {
    let s = (theta_rad * 0.5).sin();
    s * s
}

/// Inverse haversine: 2 * asin(sqrt(h)).
pub fn archav(h: f64) -> f64 {
    2.0 * h.clamp(0.0, 1.0).sqrt().asin()
}

/// Great-circle distance between two [lat, lon] points on a sphere with custom radius.
pub fn haversine_distance_with_radius(p1: [f64; 2], p2: [f64; 2], radius_m: f64) -> f64 {
    let (lat1, lat2) = (p1[0].to_radians(), p2[0].to_radians());
    let (d_lat, d_lon) = ((p2[0] - p1[0]).to_radians(), (p2[1] - p1[1]).to_radians());
    let h = hav(d_lat) + lat1.cos() * lat2.cos() * hav(d_lon);
    archav(h) * radius_m
}

/// Great-circle distance in metres using WGS84 mean Earth radius (6,371,008.8 m).
pub fn haversine_distance_m(p1: [f64; 2], p2: [f64; 2]) -> f64 {
    haversine_distance_with_radius(p1, p2, MEAN_EARTH_RADIUS_M)
}

/// Initial bearing in radians clockwise from true north in the range [0, 2*PI).
pub fn haversine_bearing_rad(from: [f64; 2], to: [f64; 2]) -> f64 {
    let (lat1, lat2) = (from[0].to_radians(), to[0].to_radians());
    let d_lon = (to[1] - from[1]).to_radians();
    let y = d_lon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * d_lon.cos();
    y.atan2(x).rem_euclid(std::f64::consts::TAU)
}

/// Initial bearing in degrees clockwise from true north in the range [0, 360).
pub fn haversine_bearing_deg(from: [f64; 2], to: [f64; 2]) -> f64 {
    haversine_bearing_rad(from, to).to_degrees()
}
