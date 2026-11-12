// Add shared ridge fixture for monotonicity tests
use sightline_cot::camera::DemGrid;
use sightline_cot::los::{check_los, compute_horizon_elevation_angle, Position3D};

fn ridge() -> DemGrid {
    let mut z = vec![0.0; 121];
    for r in 0..11 {
        z[r * 11 + 5] = 40.0;
    }
    DemGrid::new(0.0, 0.0, 100.0, 11, 11, z).unwrap()
}

fn p(e: f64, n: f64, u: f64) -> Position3D {
    Position3D { east_m: e, north_m: n, up_m: u }
}
// Add observer height monotonicity test for check_los
#[test]
fn raising_observer_never_hides_target() {
    let mut was_visible = false;
    for h in 0..=200 {
        let r = check_los(&ridge(), p(100.0, 500.0, h as f64), p(900.0, 500.0, 60.0), 40);
        assert!(r.is_visible || !was_visible);
        was_visible = r.is_visible;
    }
    assert!(was_visible);
}
// Add target height and horizon monotonicity tests
#[test]
fn raising_target_never_hides_it() {
    let mut was_visible = false;
    for h in 0..=200 {
        let r = check_los(&ridge(), p(100.0, 500.0, 60.0), p(900.0, 500.0, h as f64), 40);
        assert!(r.is_visible || !was_visible);
        was_visible = r.is_visible;
    }
}

#[test]
fn raising_observer_never_raises_horizon_angle() {
    let mut prev = f64::MAX;
    for h in 0..=100 {
        let a = compute_horizon_elevation_angle(&ridge(), p(100.0, 500.0, h as f64), core::f64::consts::FRAC_PI_2, 800.0, 40);
        assert!(a <= prev);
        prev = a;
    }
}
