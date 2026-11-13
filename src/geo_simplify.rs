//! Douglas-Peucker polyline simplification in a local frame.

const WGS84_A: f64 = 6_378_137.0;
const WGS84_E2: f64 = 0.006_694_379_990_141_316_5;

/// Local tangent frame with reference latitude and longitude.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocalFrame {
    pub origin_lat: f64,
    pub origin_lon: f64,
    pub m_lat: f64,
    pub m_lon: f64,
}

impl LocalFrame {
    /// Create a local frame at the given origin.
    pub fn new(origin_lat: f64, origin_lon: f64) -> Self {
        let p = origin_lat.to_radians();
        let s = p.sin();
        let w = 1.0 - WGS84_E2 * s * s;
        let m = WGS84_A * (1.0 - WGS84_E2) / (w * w.sqrt());
        let n = WGS84_A / w.sqrt();
        let deg = std::f64::consts::PI / 180.0;
        Self {
            origin_lat,
            origin_lon,
            m_lat: m * deg,
            m_lon: n * p.cos() * deg,
        }
    }
}
