// Add degenerate path tests for check_los
use sightline_cot::camera::DemGrid;
use sightline_cot::los::{check_los, Position3D};

fn flat() -> DemGrid {
    DemGrid::new(0.0, 0.0, 100.0, 11, 11, vec![0.0; 121]).unwrap()
}

fn p(e: f64, n: f64, u: f64) -> Position3D {
    Position3D {
        east_m: e,
        north_m: n,
        up_m: u,
    }
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
// Add below-ground observer and vertical path tests
#[test]
fn below_ground_observer_is_blocked() {
    let r = check_los(&flat(), p(100.0, 100.0, -1.0), p(900.0, 100.0, 10.0), 20);
    assert!(!r.is_visible);
    assert!(r.min_clearance_m < 0.0);
}

#[test]
fn vertical_path_above_ground_is_visible() {
    let r = check_los(&flat(), p(500.0, 500.0, 10.0), p(500.0, 500.0, 300.0), 10);
    assert!(r.is_visible);
    assert!(r.min_clearance_m >= 10.0);
}
// Add step clamp equivalence test for check_los
#[test]
fn tiny_step_counts_clamp_to_two() {
    let (a, b) = (p(100.0, 100.0, 20.0), p(900.0, 700.0, 30.0));
    let base = check_los(&flat(), a, b, 2);
    assert_eq!(check_los(&flat(), a, b, 0), base);
    assert_eq!(check_los(&flat(), a, b, 1), base);
}
