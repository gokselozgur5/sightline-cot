//! Great-circle distance and initial bearing calculations on a sphere.

/// Mean spherical Earth radius in meters (IUGG standard).
pub const EARTH_RADIUS_M: f64 = 6_371_008.8;

/// Computes great-circle distance in meters between two geodetic coordinates.
pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    haversine_distance_with_radius(lat1, lon1, lat2, lon2, EARTH_RADIUS_M)
}

/// Computes great-circle distance with a custom sphere radius in meters.
pub fn haversine_distance_with_radius(
    lat1: f64,
    lon1: f64,
    lat2: f64,
    lon2: f64,
    radius_m: f64,
) -> f64 {
    let phi1 = lat1.to_radians();
    let phi2 = lat2.to_radians();
    let delta_phi = (lat2 - lat1).to_radians();
    let delta_lambda = (lon2 - lon1).to_radians();
    let a = (delta_phi * 0.5).sin().powi(2)
        + phi1.cos() * phi2.cos() * (delta_lambda * 0.5).sin().powi(2);
    let c = 2.0 * a.clamp(0.0, 1.0).sqrt().asin();
    radius_m * c
}

/// Computes initial bearing from point 1 to point 2 in degrees in [0, 360).
pub fn initial_bearing(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    if (lat1 - lat2).abs() < 1e-12 && (lon1 - lon2).abs() < 1e-12 {
        return 0.0;
    }
    let phi1 = lat1.to_radians();
    let phi2 = lat2.to_radians();
    let delta_lambda = (lon2 - lon1).to_radians();
    let y = delta_lambda.sin() * phi2.cos();
    let x = phi1.cos() * phi2.sin() - phi1.sin() * phi2.cos() * delta_lambda.cos();
    let b = y.atan2(x).to_degrees();
    b.rem_euclid(360.0)
}

/// WGS84 semi-minor axis polar radius in meters.
pub const WGS84_SEMI_MINOR_RADIUS_M: f64 = 6_356_752.314245;

/// WGS84 flattening factor.
pub const WGS84_FLATTENING: f64 = 1.0 / 298.257223563;
