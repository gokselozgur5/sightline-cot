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
