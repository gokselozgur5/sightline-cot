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

#[test]
fn zero_shift_is_identity() {
    let s = MolodenskyShift {
        a: WGS84_A,
        f: WGS84_F,
        translation: [0.0; 3],
        da: 0.0,
        df: 0.0,
    };
    let (lat, lon, h) = s.apply(45.0, -100.0, 250.0);
    assert!((lat - 45.0).abs() < 1e-12 && (lon + 100.0).abs() < 1e-12);
    assert!((h - 250.0).abs() < 1e-9);
}

#[test]
fn wgs84_to_clarke_matches_rigorous_shift() {
    let t = [-8.0, 160.0, 176.0];
    let s = MolodenskyShift {
        a: WGS84_A,
        f: WGS84_F,
        translation: t,
        da: CLARKE_A - WGS84_A,
        df: CLARKE_F - WGS84_F,
    };
    let (lat, lon, h) = s.apply(40.0, -100.0, 100.0);
    let p = to_ecef(40.0, -100.0, 100.0, WGS84_A, WGS84_F);
    let q: [f64; 3] = std::array::from_fn(|i| p[i] + t[i]);
    let r = to_ecef(lat, lon, h, CLARKE_A, CLARKE_F);
    let d = ((q[0] - r[0]).powi(2) + (q[1] - r[1]).powi(2) + (q[2] - r[2]).powi(2)).sqrt();
    assert!(d < 1.0, "residual {d} m");
}
