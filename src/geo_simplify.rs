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

impl LocalFrame {
    /// Project [lat, lon] to [east_m, north_m] in the local frame.
    pub fn project(&self, pt: [f64; 2]) -> [f64; 2] {
        let d_lat = pt[0] - self.origin_lat;
        let d_lon = (pt[1] - self.origin_lon + 540.0).rem_euclid(360.0) - 180.0;
        [d_lon * self.m_lon, d_lat * self.m_lat]
    }

    /// Unproject [east_m, north_m] to [lat, lon] in degrees.
    pub fn unproject(&self, xy: [f64; 2]) -> [f64; 2] {
        let d_lat = if self.m_lat.abs() > 1e-12 { xy[1] / self.m_lat } else { 0.0 };
        let d_lon = if self.m_lon.abs() > 1e-12 { xy[0] / self.m_lon } else { 0.0 };
        [self.origin_lat + d_lat, self.origin_lon + d_lon]
    }
}

/// Distance in metres from point p to segment [a, b].
pub fn segment_distance(p: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    let len_sq = dx * dx + dy * dy;
    if len_sq <= 1e-12 {
        let (ex, ey) = (p[0] - a[0], p[1] - a[1]);
        return (ex * ex + ey * ey).sqrt();
    }
    let t = (((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / len_sq).clamp(0.0, 1.0);
    let (cx, cy) = (a[0] + t * dx, a[1] + t * dy);
    let (rx, ry) = (p[0] - cx, p[1] - cy);
    (rx * rx + ry * ry).sqrt()
}
