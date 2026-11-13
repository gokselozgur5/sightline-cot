//! Helmert transform identity and round-trip tests.

use sightline_cot::geo_helmert::HelmertParams;

const P: [f64; 3] = [4_000_000.0, 3_000_000.0, 3_800_000.0];

#[test]
fn zero_parameters_are_identity() {
    let h = HelmertParams::default();
    assert_eq!(h.apply(P), P);
    assert_eq!(h.apply_inverse(P), P);
}

#[test]
fn translation_only_shifts_point() {
    let h = HelmertParams {
        translation: [1.0, -2.0, 3.0],
        ..Default::default()
    };
    assert_eq!(h.apply([0.0; 3]), [1.0, -2.0, 3.0]);
}

#[test]
fn inverse_round_trip_within_micrometre() {
    let h = HelmertParams {
        translation: [-0.0127, -0.0029, 0.0023],
        rotation_arcsec: [0.00395, 0.00453, -0.0237],
        scale_ppm: 0.62,
    };
    for k in 0..8 {
        let f = k as f64 * 0.5 - 1.5;
        let p = [P[0] * f, P[1] - 1.0e6 * f, P[2] + 2.0e5 * f];
        let r = h.apply_inverse(h.apply(p));
        for i in 0..3 {
            assert!((r[i] - p[i]).abs() < 1e-6, "axis {i}: {} vs {}", r[i], p[i]);
        }
    }
}
