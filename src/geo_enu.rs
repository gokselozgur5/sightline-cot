//! Geodetic to ENU (East-North-Up) conversion.

/// Error indicating failure during geodetic to ENU conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnuError {
    /// Non-finite coordinate or parameter value encountered.
    NonFinite,
    /// Latitude outside the valid range [-90, 90] degrees.
    LatitudeOutOfRange,
}

impl std::fmt::Display for EnuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFinite => write!(f, "non-finite coordinate or parameter"),
            Self::LatitudeOutOfRange => write!(f, "latitude must be between -90 and 90 degrees"),
        }
    }
}

impl std::error::Error for EnuError {}

use crate::geo_ecef::geodetic_to_ecef;

fn ecef_to_enu(p_ecef: [f64; 3], ref_lat: f64, ref_lon: f64, ref_hae: f64) -> [f64; 3] {
    let ref_ecef = geodetic_to_ecef(ref_lat, ref_lon, ref_hae);
    let (s_lat, c_lat) = (ref_lat.to_radians().sin(), ref_lat.to_radians().cos());
    let (s_lon, c_lon) = (ref_lon.to_radians().sin(), ref_lon.to_radians().cos());
    let (dx, dy, dz) = (
        p_ecef[0] - ref_ecef[0],
        p_ecef[1] - ref_ecef[1],
        p_ecef[2] - ref_ecef[2],
    );
    let e = -s_lon * dx + c_lon * dy;
    let n = -s_lat * c_lon * dx - s_lat * s_lon * dy + c_lat * dz;
    let u = c_lat * c_lon * dx + c_lat * s_lon * dy + s_lat * dz;
    [e, n, u]
}

/// Convert Geodetic coordinates (lat, lon, hae) to local ENU offset from reference origin.
pub fn geodetic_to_enu(
    lat_deg: f64,
    lon_deg: f64,
    hae_m: f64,
    ref_lat_deg: f64,
    ref_lon_deg: f64,
    ref_hae_m: f64,
) -> Result<[f64; 3], EnuError> {
    if !lat_deg.is_finite()
        || !lon_deg.is_finite()
        || !hae_m.is_finite()
        || !ref_lat_deg.is_finite()
        || !ref_lon_deg.is_finite()
        || !ref_hae_m.is_finite()
    {
        return Err(EnuError::NonFinite);
    }
    if !(-90.0..=90.0).contains(&lat_deg) || !(-90.0..=90.0).contains(&ref_lat_deg) {
        return Err(EnuError::LatitudeOutOfRange);
    }
    let p_ecef = geodetic_to_ecef(lat_deg, lon_deg, hae_m);
    Ok(ecef_to_enu(p_ecef, ref_lat_deg, ref_lon_deg, ref_hae_m))
}

pub use crate::camera::enu_to_geodetic;
