//! Geodetic LocalFrame with cached reference trigonometry.

/// Error indicating failure during LocalFrame creation or projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeoFrameError {
    /// Non-finite coordinate or parameter value encountered.
    NonFinite,
    /// Latitude outside the valid range [-90, 90] degrees.
    LatitudeOutOfRange,
}

impl std::fmt::Display for GeoFrameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFinite => write!(f, "non-finite coordinate or parameter"),
            Self::LatitudeOutOfRange => write!(f, "latitude must be between -90 and 90 degrees"),
        }
    }
}

impl std::error::Error for GeoFrameError {}

const WGS84_A: f64 = 6378137.0;
const WGS84_F: f64 = 1.0 / 298.257223563;
const WGS84_B: f64 = WGS84_A * (1.0 - WGS84_F);
const WGS84_E2: f64 = 2.0 * WGS84_F - WGS84_F * WGS84_F;
const WGS84_E_PRIME2: f64 = (WGS84_A * WGS84_A - WGS84_B * WGS84_B) / (WGS84_B * WGS84_B);

pub(crate) fn geodetic_to_ecef(lat_deg: f64, lon_deg: f64, hae_m: f64) -> [f64; 3] {
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

pub(crate) fn ecef_to_geodetic(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let p = (x * x + y * y).sqrt();
    let lon_rad = y.atan2(x);
    let theta = (z * WGS84_A).atan2(p * WGS84_B);
    let (s_th, c_th) = (theta.sin(), theta.cos());
    let num = z + WGS84_E_PRIME2 * WGS84_B * s_th * s_th * s_th;
    let den = p - WGS84_E2 * WGS84_A * c_th * c_th * c_th;
    let lat_rad = num.atan2(den);
    let s_lat = lat_rad.sin();
    let n = WGS84_A / (1.0 - WGS84_E2 * s_lat * s_lat).sqrt();
    let hae_m = if p > 1e-6 {
        p / lat_rad.cos() - n
    } else {
        z.abs() - WGS84_B
    };
    (lat_rad.to_degrees(), lon_rad.to_degrees(), hae_m)
}
