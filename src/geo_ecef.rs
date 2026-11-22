//! WGS84 Geodetic to ECEF and ECEF to Geodetic coordinate conversions.

use crate::ellipsoid::Ellipsoid;

pub const WGS84_A: f64 = Ellipsoid::WGS84.a;
pub const WGS84_F: f64 = Ellipsoid::WGS84.f;
pub const WGS84_B: f64 = Ellipsoid::WGS84.b;
pub const WGS84_E2: f64 = Ellipsoid::WGS84.f * (2.0 - Ellipsoid::WGS84.f);
pub const WGS84_E_PRIME2: f64 = (WGS84_A * WGS84_A - WGS84_B * WGS84_B) / (WGS84_B * WGS84_B);

/// Convert geodetic coordinates to WGS84 ECEF coordinates in meters.
pub fn geodetic_to_ecef(lat_deg: f64, lon_deg: f64, hae_m: f64) -> [f64; 3] {
    let lat_rad = lat_deg.to_radians();
    let lon_rad = lon_deg.to_radians();
    let (s_lat, c_lat) = (lat_rad.sin(), lat_rad.cos());
    let (s_lon, c_lon) = (lon_rad.sin(), lon_rad.cos());
    let n = WGS84_A / (1.0 - WGS84_E2 * s_lat * s_lat).sqrt();
    let x = (n + hae_m) * c_lat * c_lon;
    let y = (n + hae_m) * c_lat * s_lon;
    let z = (n * (1.0 - WGS84_E2) + hae_m) * s_lat;
    [x, y, z]
}

/// Convert WGS84 ECEF coordinates in meters to geodetic coordinates.
pub fn ecef_to_geodetic(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let p = (x * x + y * y).sqrt();
    let lon_rad = y.atan2(x);
    let theta = (z * WGS84_A).atan2(p * WGS84_B);
    let (s_th, c_th) = (theta.sin(), theta.cos());
    let num = z + WGS84_E_PRIME2 * WGS84_B * s_th * s_th * s_th;
    let den = p - WGS84_E2 * WGS84_A * c_th * c_th * c_th;
    let raw_lat = num.atan2(den);
    let half_pi = core::f64::consts::FRAC_PI_2;
    let lat_rad = if raw_lat > half_pi {
        core::f64::consts::PI - raw_lat
    } else if raw_lat < -half_pi {
        -core::f64::consts::PI - raw_lat
    } else {
        raw_lat
    };
    let s_lat = lat_rad.sin();
    let n = WGS84_A / (1.0 - WGS84_E2 * s_lat * s_lat).sqrt();
    let hae_m = if p > 1e-6 {
        p / lat_rad.cos() - n
    } else {
        z.abs() - WGS84_B
    };
    (lat_rad.to_degrees(), lon_rad.to_degrees(), hae_m)
}

/// Convert ECEF coordinate array `[x, y, z]` in meters to geodetic coordinates.
pub fn ecef_point_to_geodetic(p: [f64; 3]) -> (f64, f64, f64) {
    ecef_to_geodetic(p[0], p[1], p[2])
}
