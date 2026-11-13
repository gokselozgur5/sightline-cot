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
