use sightline_cot::camera::DemGrid;
use sightline_cot::los::{check_los, Position3D};

#[test]
fn test_target_under_terrain_is_blocked() {
    let elev = vec![0.0, 0.0, 0.0, 0.0, 20.0, 0.0, 0.0, 0.0, 0.0];
    let dem = DemGrid::new(0.0, 0.0, 10.0, 3, 3, elev).unwrap();
    let obs = Position3D {
        east_m: 0.0,
        north_m: 10.0,
        up_m: 1000.0,
    };
    let tgt = Position3D {
        east_m: 10.0,
        north_m: 10.0,
        up_m: 5.0,
    };
    let res = check_los(&dem, obs, tgt, 10);
    assert!(!res.is_visible);
    assert!(res.min_clearance_m < 0.0);
}
