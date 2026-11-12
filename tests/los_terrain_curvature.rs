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
// Add curvature blocking test for flat long paths
use sightline_cot::camera::DemGrid;
use sightline_cot::los::{check_los, Position3D};

fn flat() -> DemGrid {
    DemGrid::new(0.0, 0.0, 1000.0, 11, 11, vec![0.0; 121]).unwrap()
}

fn p(e: f64, n: f64, u: f64) -> Position3D {
    Position3D {
        east_m: e,
        north_m: n,
        up_m: u,
    }
}

#[test]
fn low_endpoints_over_10km_are_blocked_by_curvature() {
    let r = check_los(&flat(), p(0.0, 5000.0, 1.5), p(10_000.0, 5000.0, 1.5), 10);
    assert!(!r.is_visible);
}

#[test]
fn higher_endpoints_clear_curvature_with_expected_margin() {
    let r = check_los(&flat(), p(0.0, 5000.0, 10.0), p(10_000.0, 5000.0, 10.0), 10);
    let expect = 10.0 - earth_curvature_drop(10_000.0) / 4.0;
    assert!(r.is_visible);
    assert!((r.min_clearance_m - expect).abs() < 1e-6);
}
// Add horizon angle tests on flat terrain
use sightline_cot::los::compute_horizon_elevation_angle;

#[test]
fn flat_horizon_angle_matches_curvature_formula() {
    let obs = p(500.0, 500.0, 10.0);
    let ang = compute_horizon_elevation_angle(&flat(), obs, 0.0, 1000.0, 1);
    let expect = (-earth_curvature_drop(1000.0) - 10.0).atan2(1000.0);
    assert!((ang - expect).abs() < 1e-12);
}

#[test]
fn horizon_without_terrain_data_is_straight_down() {
    let obs = p(-5000.0, -5000.0, 10.0);
    let ang = compute_horizon_elevation_angle(&flat(), obs, 0.0, 100.0, 4);
    assert_eq!(ang, -core::f64::consts::FRAC_PI_2);
}
