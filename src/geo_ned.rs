//! Local ENU (East-North-Up) and NED (North-East-Down) frame conversions.

pub use crate::geo_enu::EnuError;

/// Rotate a 3D vector from ENU (East, North, Up) to NED (North, East, Down).
#[inline]
pub fn enu_to_ned(enu: [f64; 3]) -> [f64; 3] {
    [enu[1], enu[0], -enu[2]]
}

/// Rotate a 3D vector from NED (North, East, Down) to ENU (East, North, Up).
#[inline]
pub fn ned_to_enu(ned: [f64; 3]) -> [f64; 3] {
    [ned[1], ned[0], -ned[2]]
}

/// Convert Geodetic coordinates (lat, lon, hae) to local NED offset from reference origin.
pub fn geodetic_to_ned(
    lat_deg: f64,
    lon_deg: f64,
    hae_m: f64,
    ref_lat_deg: f64,
    ref_lon_deg: f64,
    ref_hae_m: f64,
) -> Result<[f64; 3], EnuError> {
    crate::geo_enu::geodetic_to_enu(lat_deg, lon_deg, hae_m, ref_lat_deg, ref_lon_deg, ref_hae_m)
        .map(enu_to_ned)
}

/// Convert local NED offset from reference origin to Geodetic coordinates (lat, lon, hae).
pub fn ned_to_geodetic(
    ned: [f64; 3],
    ref_lat_deg: f64,
    ref_lon_deg: f64,
    ref_hae_m: f64,
) -> (f64, f64, f64) {
    crate::camera::enu_to_geodetic(ned_to_enu(ned), ref_lat_deg, ref_lon_deg, ref_hae_m)
}
