use sightline_cot::geo_helmert::HelmertParams;

const P: [f64; 3] = [4_000_000.0, 3_000_000.0, 4_500_000.0];

#[test]
fn zero_parameters_are_identity() {
    assert_eq!(HelmertParams::identity().apply(P), P);
}

#[test]
fn translation_only_adds_offsets() {
    let h = HelmertParams { translation_m: [1.0, -2.0, 3.0], ..Default::default() };
    assert_eq!(h.apply([0.0; 3]), [1.0, -2.0, 3.0]);
}

#[test]
fn apply_then_invert_round_trips() {
    let h = HelmertParams {
        translation_m: [0.067, 0.517, 0.223],
        rotation_arcsec: [0.0, 0.0, 0.0057],
        scale_ppm: -0.0134,
    };
    let big = HelmertParams { rotation_arcsec: [0.8, -1.2, 2.0], scale_ppm: 12.0, ..h };
    for params in [h, big] {
        let back = params.invert(params.apply(P));
        for i in 0..3 {
            assert!((back[i] - P[i]).abs() < 1e-6);
        }
    }
}
