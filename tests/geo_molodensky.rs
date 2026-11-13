use sightline_cot::geo_molodensky::MolodenskyShift;

const WGS84_A: f64 = 6_378_137.0;
const WGS84_F: f64 = 1.0 / 298.257_223_563;
const CLARKE_A: f64 = 6_378_206.4;
const CLARKE_F: f64 = 1.0 / 294.978_698_2;

fn nad27() -> MolodenskyShift {
    MolodenskyShift {
        a: WGS84_A,
        f: WGS84_F,
        da: CLARKE_A - WGS84_A,
        df: CLARKE_F - WGS84_F,
        dxyz: [8.0, -160.0, -176.0],
    }
}

#[test]
fn zero_shift_is_identity() {
    let z = MolodenskyShift {
        da: 0.0,
        df: 0.0,
        dxyz: [0.0; 3],
        ..nad27()
    };
    let (la, lo, h) = z.apply(40.0, -100.0, 250.0);
    assert!((la - 40.0).abs() < 1e-12 && (lo + 100.0).abs() < 1e-12 && (h - 250.0).abs() < 1e-9);
}

fn to_ecef(a: f64, f: f64, lat: f64, lon: f64, h: f64) -> [f64; 3] {
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
fn matches_exact_ecef_shift_within_one_meter() {
    let s = nad27();
    let (lat, lon, h) = (40.0, -100.0, 200.0);
    let (la, lo, hh) = s.apply(lat, lon, h);
    let src = to_ecef(WGS84_A, WGS84_F, lat, lon, h);
    let moved = [src[0] + 8.0, src[1] - 160.0, src[2] - 176.0];
    // Exact target-datum ECEF of the Molodensky result must equal the moved point.
    let got = to_ecef(CLARKE_A, CLARKE_F, la, lo, hh);
    for i in 0..3 {
        assert!((got[i] - moved[i]).abs() < 1.0, "axis {i}");
    }
}
