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

const WGS84_A: f64 = 6378137.0;
const WGS84_F: f64 = 1.0 / 298.257223563;
const WGS84_E2: f64 = 2.0 * WGS84_F - WGS84_F * WGS84_F;

fn geodetic_to_ecef(lat_deg: f64, lon_deg: f64, hae_m: f64) -> [f64; 3] {
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
