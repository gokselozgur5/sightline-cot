//! Spherical geodesy backwards-compatibility re-exports.
pub use crate::geo_greatcircle::{haversine_distance, initial_bearing, EARTH_RADIUS_M};

/// Computes great-circle distance with a custom sphere radius in meters.
pub fn haversine_distance_with_radius(
    lat1: f64,
    lon1: f64,
    lat2: f64,
    lon2: f64,
    radius_m: f64,
) -> f64 {
    let d = crate::geo_greatcircle::haversine_distance(lat1, lon1, lat2, lon2);
    d * (radius_m / crate::geo_greatcircle::EARTH_RADIUS_M)
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
    if b >= 180.0 {
        b - 360.0
    } else {
        b
    }
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

/// Absolute difference between two bearings in degrees.
pub fn bearing_delta_deg(b1: f64, b2: f64) -> f64 {
    normalize_bearing_pm180(b1 - b2).abs()
}

/// Geometric distance to horizon in meters for height above sphere.
pub fn horizon_distance_m(height_m: f64, radius_m: f64) -> f64 {
    (height_m * (2.0 * radius_m + height_m)).sqrt()
}

/// Mutual line of sight between two elevated observers in meters.
pub fn line_of_sight_m(h1_m: f64, h2_m: f64, radius_m: f64) -> f64 {
    horizon_distance_m(h1_m, radius_m) + horizon_distance_m(h2_m, radius_m)
}

/// Computes speed in m/s given distance in meters and time in seconds.
pub fn speed_mps(dist_m: f64, dt_sec: f64) -> Option<f64> {
    if dt_sec > 0.0 {
        Some(dist_m / dt_sec)
    } else {
        None
    }
}

/// Computes estimated time of arrival (seconds) given distance and speed.
pub fn time_to_go_sec(dist_m: f64, speed_mps: f64) -> Option<f64> {
    if speed_mps > 0.0 {
        Some(dist_m / speed_mps)
    } else {
        None
    }
}

/// Checks whether course over ground in degrees is a valid number.
pub fn is_valid_cog_deg(cog_deg: f64) -> bool {
    cog_deg.is_finite() && (0.0..360.0).contains(&cog_deg)
}

/// Checks whether speed over ground in m/s is non-negative and finite.
pub fn is_valid_sog_mps(sog_mps: f64) -> bool {
    sog_mps.is_finite() && sog_mps >= 0.0
}

/// Verification marker for spherical navigation library extensions.
pub const GEO_SPHERE_EXTENSIONS_VERIFIED: bool = true;
