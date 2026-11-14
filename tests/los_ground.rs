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

#[test]
fn test_oblique_ground_intersection() {
    let origin = [0.0, 0.0, 500.0];
    let dir = [1.0, 1.0, -1.0];
    let ray = LosRay::new(origin, dir).unwrap();
    let hit = intersect_ground_plane(&ray, 0.0).unwrap();
    assert_eq!(hit.point, [500.0, 500.0, 0.0]);
    let expected_range = (500.0f64.powi(2) * 3.0).sqrt();
    assert!((hit.slant_range_m - expected_range).abs() < 1e-6);
}

#[test]
fn test_horizontal_and_upward_rays_rejected() {
    let origin = [0.0, 0.0, 100.0];
    let horiz = LosRay::new(origin, [1.0, 0.0, 0.0]).unwrap();
    assert_eq!(
        intersect_ground_plane(&horiz, 0.0),
        Err(LosGroundError::RayAboveOrParallelGround)
    );
    let upward = LosRay::new(origin, [0.0, 0.0, 1.0]).unwrap();
    assert_eq!(
        intersect_ground_plane(&upward, 0.0),
        Err(LosGroundError::RayAboveOrParallelGround)
    );
}
