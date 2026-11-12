// Add degenerate path tests for check_los
use sightline_cot::camera::DemGrid;
use sightline_cot::los::{check_los, Position3D};

fn flat() -> DemGrid {
    DemGrid::new(0.0, 0.0, 100.0, 11, 11, vec![0.0; 121]).unwrap()
}

fn p(e: f64, n: f64, u: f64) -> Position3D {
    Position3D { east_m: e, north_m: n, up_m: u }
}

#[test]
fn same_point_above_ground_is_visible() {
    let r = check_los(&flat(), p(500.0, 500.0, 50.0), p(500.0, 500.0, 50.0), 10);
    assert!(r.is_visible);
    assert!((r.min_clearance_m - 50.0).abs() < 1e-9);
}

#[test]
fn same_point_below_ground_is_blocked() {
    let r = check_los(&flat(), p(500.0, 500.0, -5.0), p(500.0, 500.0, -5.0), 10);
    assert!(!r.is_visible);
    assert_eq!(r.obstruction.unwrap().up_m, 0.0);
}
