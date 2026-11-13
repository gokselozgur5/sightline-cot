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
    let z = MolodenskyShift { da: 0.0, df: 0.0, dxyz: [0.0; 3], ..nad27() };
    let (la, lo, h) = z.apply(40.0, -100.0, 250.0);
    assert!((la - 40.0).abs() < 1e-12 && (lo + 100.0).abs() < 1e-12 && (h - 250.0).abs() < 1e-9);
}
