//! Great-circle distance and initial bearing calculations on a sphere.

/// Mean spherical Earth radius in meters (IUGG standard).
pub const EARTH_RADIUS_M: f64 = 6_371_008.8;

/// Computes great-circle distance in meters between two geodetic coordinates.
pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let phi1 = lat1.to_radians();
    let phi2 = lat2.to_radians();
    let delta_phi = (lat2 - lat1).to_radians();
    let delta_lambda = (lon2 - lon1).to_radians();
    let a = (delta_phi * 0.5).sin().powi(2)
        + phi1.cos() * phi2.cos() * (delta_lambda * 0.5).sin().powi(2);
    let c = 2.0 * a.clamp(0.0, 1.0).sqrt().asin();
    EARTH_RADIUS_M * c
}
