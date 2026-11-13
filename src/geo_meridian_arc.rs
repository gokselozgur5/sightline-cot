//! WGS84 meridian arc length between two geodetic latitudes.

const WGS84_A: f64 = 6_378_137.0;
const WGS84_E2: f64 = 0.006_694_379_990_141_316_5;

/// Distance along the meridian from the equator to `lat` radians, in metres.
fn distance_from_equator(lat: f64) -> f64 {
    let e2 = WGS84_E2;
    let (e4, e6, e8) = (e2 * e2, e2 * e2 * e2, e2 * e2 * e2 * e2);
    let a0 = 1.0 + 0.75 * e2 + 45.0 / 64.0 * e4 + 175.0 / 256.0 * e6 + 11025.0 / 16384.0 * e8;
    let a2 = 0.75 * e2 + 15.0 / 16.0 * e4 + 525.0 / 512.0 * e6 + 2205.0 / 2048.0 * e8;
    let a4 = 15.0 / 64.0 * e4 + 105.0 / 256.0 * e6 + 2205.0 / 4096.0 * e8;
    let a6 = 35.0 / 512.0 * e6 + 315.0 / 2048.0 * e8;
    let a8 = 315.0 / 16384.0 * e8;
    let s = a0 * lat - a2 / 2.0 * (2.0 * lat).sin() + a4 / 4.0 * (4.0 * lat).sin()
        - a6 / 6.0 * (6.0 * lat).sin()
        + a8 / 8.0 * (8.0 * lat).sin();
    WGS84_A * (1.0 - e2) * s
}

/// Signed meridian arc length in metres from `lat1_deg` to `lat2_deg`.
pub fn meridian_arc(lat1_deg: f64, lat2_deg: f64) -> f64 {
    distance_from_equator(lat2_deg.to_radians()) - distance_from_equator(lat1_deg.to_radians())
}
