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
