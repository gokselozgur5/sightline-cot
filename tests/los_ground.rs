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

#[test]
fn test_non_finite_and_degenerate_inputs_rejected() {
    assert_eq!(
        LosRay::new([f64::NAN, 0.0, 0.0], [0.0, 0.0, -1.0]),
        Err(LosGroundError::NonFiniteInput)
    );
    assert_eq!(
        LosRay::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
        Err(LosGroundError::DegenerateRayDirection)
    );
    let valid_ray = LosRay::new([0.0, 0.0, 100.0], [0.0, 0.0, -1.0]).unwrap();
    assert_eq!(
        intersect_ground_plane(&valid_ray, f64::INFINITY),
        Err(LosGroundError::NonFiniteInput)
    );
}

use sightline_cot::los_ground::intersect_sloped_plane;

#[test]
fn test_sloped_plane_intersection() {
    let origin = [0.0, 0.0, 100.0];
    let dir = [0.0, 0.0, -1.0];
    let ray = LosRay::new(origin, dir).unwrap();
    let hit = intersect_sloped_plane(&ray, [0.0, 0.0, 10.0], 0.1, 0.0).unwrap();
    assert_eq!(hit.point, [0.0, 0.0, 10.0]);
    assert!((hit.slant_range_m - 90.0).abs() < 1e-9);
}

use sightline_cot::camera::CameraPose;
use sightline_cot::los_ground::camera_los_ground;

#[test]
fn test_camera_los_ground_nadir() {
    let pose = CameraPose {
        roll_rad: 0.0,
        pitch_rad: 0.0,
        yaw_rad: 0.0,
        cam_up_m: 500.0,
    };
    let hit = camera_los_ground(&pose, [0.0, 0.0, 1.0], 0.0).unwrap();
    assert!((hit.point[0]).abs() < 1e-9);
    assert!((hit.point[1]).abs() < 1e-9);
    assert!((hit.point[2]).abs() < 1e-9);
    assert!((hit.slant_range_m - 500.0).abs() < 1e-9);
}
