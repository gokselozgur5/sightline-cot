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

/// WGS84 first eccentricity squared.
pub const WGS84_FIRST_ECCENTRICITY_SQ: f64 = 0.00669437999014;

/// WGS84 second eccentricity squared.
pub const WGS84_SECOND_ECCENTRICITY_SQ: f64 = 0.00673949674228;

/// GRS80 semi-major axis in meters.
pub const GRS80_SEMI_MAJOR_RADIUS_M: f64 = 6_378_137.0;

/// GRS80 flattening factor.
pub const GRS80_FLATTENING: f64 = 1.0 / 298.257222101;

/// Standard nautical mile in meters.
pub const NAUTICAL_MILE_M: f64 = 1852.0;

/// International foot to meters conversion factor.
pub const FEET_TO_METERS: f64 = 0.3048;

/// Meters to international feet conversion factor.
pub const METERS_TO_FEET: f64 = 1.0 / 0.3048;

/// Knots to meters per second conversion factor.
pub const KNOTS_TO_MPS: f64 = 1852.0 / 3600.0;

/// Meters per second to knots conversion factor.
pub const MPS_TO_KNOTS: f64 = 3600.0 / 1852.0;

/// Degrees to radians factor.
pub fn deg_to_rad_fast(deg: f64) -> f64 {
    deg * (std::f64::consts::PI / 180.0)
}

/// Radians to degrees factor.
pub fn rad_to_deg_fast(rad: f64) -> f64 {
    rad * (180.0 / std::f64::consts::PI)
}

/// Normalizes bearing in degrees to [0, 360).
pub fn normalize_bearing_360(bearing: f64) -> f64 {
    (bearing % 360.0 + 360.0) % 360.0
}

/// Normalizes relative angle in degrees to [-180, 180).
pub fn normalize_bearing_pm180(bearing: f64) -> f64 {
    let b = normalize_bearing_360(bearing);
    if b >= 180.0 { b - 360.0 } else { b }
}

/// Converts meters to international nautical miles.
pub fn meters_to_nm(meters: f64) -> f64 {
    meters / NAUTICAL_MILE_M
}

/// Converts international nautical miles to meters.
pub fn nm_to_meters(nm: f64) -> f64 {
    nm * NAUTICAL_MILE_M
}

/// Great-circle angular distance in radians from distance and sphere radius.
pub fn angular_distance_rad(dist_m: f64, radius_m: f64) -> f64 {
    dist_m / radius_m
}

/// Checks whether latitude in degrees is valid [-90, 90].
pub fn is_valid_latitude_deg(lat_deg: f64) -> bool {
    (-90.0..=90.0).contains(&lat_deg)
}

/// Checks whether longitude in degrees is valid [-180, 180].
pub fn is_valid_longitude_deg(lon_deg: f64) -> bool {
    (-180.0..=180.0).contains(&lon_deg)
}

/// Spherical chord length between two points from angular separation.
pub fn chord_length_m(angular_rad: f64, radius_m: f64) -> f64 {
    2.0 * radius_m * (angular_rad / 2.0).sin()
}

/// Angular subtense of an object of given size at given distance.
pub fn subtense_rad(size_m: f64, dist_m: f64) -> f64 {
    (size_m / dist_m).atan()
}
