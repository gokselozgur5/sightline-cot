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
