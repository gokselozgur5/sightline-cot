//! WGS84 radii of curvature as functions of geodetic latitude.

const WGS84_A: f64 = 6_378_137.0;
const WGS84_E2: f64 = 0.006_694_379_990_141_316_5;

/// Meridian radius of curvature M in metres at `lat_deg` degrees.
pub fn meridian_radius(lat_deg: f64) -> f64 {
    let s = lat_deg.to_radians().sin();
    let w = 1.0 - WGS84_E2 * s * s;
    WGS84_A * (1.0 - WGS84_E2) / (w * w.sqrt())
}
