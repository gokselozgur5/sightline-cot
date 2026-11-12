// Line-of-sight must not depend on which endpoint is the observer.
use sightline_cot::camera::DemGrid;
use sightline_cot::los::{check_los, earth_curvature_drop, Position3D};

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
fn swapping_endpoints_gives_same_visibility() {
    let (a, b) = (p(0.0, 5000.0, 20.0), p(10_000.0, 5000.0, 2.0));
    let ab = check_los(&flat(), a, b, 100).is_visible;
    assert_eq!(ab, check_los(&flat(), b, a, 100).is_visible);
}

#[test]
fn five_metre_masts_10km_apart_are_visible() {
    let r = check_los(&flat(), p(0.0, 5000.0, 5.0), p(10_000.0, 5000.0, 5.0), 100);
    let expect = 5.0 - earth_curvature_drop(10_000.0) / 4.0;
    assert!(r.is_visible);
    assert!((r.min_clearance_m - expect).abs() < 1e-6);
}
