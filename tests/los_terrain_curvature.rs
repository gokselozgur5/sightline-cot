// Add earth_curvature_drop scaling tests
use sightline_cot::los::earth_curvature_drop;

#[test]
fn curvature_drop_is_zero_for_non_positive_distance() {
    assert_eq!(earth_curvature_drop(0.0), 0.0);
    assert_eq!(earth_curvature_drop(-500.0), 0.0);
}

#[test]
fn curvature_drop_scales_quadratically() {
    let d = earth_curvature_drop(3_000.0);
    assert!((earth_curvature_drop(6_000.0) - 4.0 * d).abs() < 1e-9);
}
