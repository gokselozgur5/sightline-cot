//! Abridged Molodensky tests against the identity and a rigorous ECEF shift.

use sightline_cot::geo_molodensky::MolodenskyShift;

const WGS84_A: f64 = 6_378_137.0;
const WGS84_F: f64 = 1.0 / 298.257_223_563;
const CLARKE_A: f64 = 6_378_206.4;
const CLARKE_F: f64 = 1.0 / 294.978_698_2;

fn to_ecef(lat: f64, lon: f64, h: f64, a: f64, f: f64) -> [f64; 3] {
    let (p, l) = (lat.to_radians(), lon.to_radians());
    let e2 = f * (2.0 - f);
    let n = a / (1.0 - e2 * p.sin().powi(2)).sqrt();
    [
        (n + h) * p.cos() * l.cos(),
        (n + h) * p.cos() * l.sin(),
        (n * (1.0 - e2) + h) * p.sin(),
    ]
}
