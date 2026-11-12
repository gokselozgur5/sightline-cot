// Add outside-grid path tests for check_los
use sightline_cot::camera::DemGrid;
use sightline_cot::los::{check_los, compute_horizon_elevation_angle, Position3D};

fn flat() -> DemGrid {
    DemGrid::new(0.0, 0.0, 100.0, 11, 11, vec![0.0; 121]).unwrap()
}

fn p(e: f64, n: f64, u: f64) -> Position3D {
    Position3D { east_m: e, north_m: n, up_m: u }
}

#[test]
fn path_fully_outside_grid_is_visible_with_max_clearance() {
    let r = check_los(&flat(), p(2000.0, 2000.0, -50.0), p(3000.0, 2000.0, -50.0), 10);
    assert!(r.is_visible);
    assert_eq!(r.min_clearance_m, f64::MAX);
}
// Add grid-leaving path and edge endpoint tests
#[test]
fn only_in_grid_samples_can_block_a_leaving_path() {
    let r = check_los(&flat(), p(500.0, 500.0, 10.0), p(5000.0, 500.0, 10.0), 50);
    assert!(r.is_visible);
    assert!(r.min_clearance_m > 0.0 && r.min_clearance_m < f64::MAX);
}

#[test]
fn endpoint_on_grid_edge_is_handled() {
    let r = check_los(&flat(), p(0.0, 0.0, 10.0), p(1000.0, 1000.0, 10.0), 10);
    assert!(r.is_visible);
}
