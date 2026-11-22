use sightline_cot::imm::{ImmConfig, ImmFilter};

#[test]
fn step_with_nan_dt_does_not_poison_the_filter() {
    let mut imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    imm.step(f64::NAN, [1.0, 1.0, 1.0]);
    let fused = imm.step(0.1, [1.0, 1.0, 1.0]);
    assert!(fused.iter().all(|v| v.is_finite()), "{fused:?}");
}
