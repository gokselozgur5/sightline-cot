use sightline_cot::los_ground::{
    intersect_ground_plane, LosGroundError, LosRay,
};

#[test]
fn test_nadir_ground_intersection() {
    let origin = [100.0, 200.0, 1000.0];
    let dir = [0.0, 0.0, -1.0];
    let ray = LosRay::new(origin, dir).unwrap();
    let hit = intersect_ground_plane(&ray, 0.0).unwrap();
    assert_eq!(hit.point, [100.0, 200.0, 0.0]);
    assert!((hit.slant_range_m - 1000.0).abs() < 1e-9);
    assert!((hit.t - 1000.0).abs() < 1e-9);
}
