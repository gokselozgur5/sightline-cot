use sightline_cot::camera::{estimate_covariance_ce_le, CameraPose, Intrinsics};

#[test]
fn test_ce90_le90_zero_and_nonzero_sigmas() {
    let cam = Intrinsics::Pinhole {
        fx: 1000.0,
        fy: 1000.0,
        cx: 500.0,
        cy: 500.0,
    };
    let pose = CameraPose {
        roll_rad: 0.0,
        pitch_rad: 0.0,
        yaw_rad: 0.0,
        cam_up_m: 100.0,
    };
    let (ce0, le0) = estimate_covariance_ce_le(&cam, &pose, 500.0, 500.0, 0.0, 0.0, 0.0).unwrap();
    assert_eq!(ce0, 0.0);
    assert_eq!(le0, 0.0);
    let (ce1, le1) = estimate_covariance_ce_le(&cam, &pose, 500.0, 500.0, 0.0, 1.0, 0.001).unwrap();
    assert!(ce1 > 0.0);
    assert!(le1 > 0.0);
}

#[test]
fn test_camera_pose_roll_tait_bryan_displacement() {
    let pose = CameraPose {
        roll_rad: 0.1,
        pitch_rad: 0.0,
        yaw_rad: 0.0,
        cam_up_m: 100.0,
    };
    let pt = sightline_cot::camera::intersect_ground([0.0, 0.0, 1.0], &pose, 0.0).unwrap();
    assert!(pt[0].abs() > 9.0);
}

#[test]
fn test_pinhole_unproject_squared_norm_overflow() {
    let cam = Intrinsics::Pinhole {
        fx: 1e-160,
        fy: 1e-160,
        cx: 0.0,
        cy: 0.0,
    };
    let ray = cam.pixel_to_ray(1.0, 0.0).unwrap();
    let norm = (ray[0] * ray[0] + ray[1] * ray[1] + ray[2] * ray[2]).sqrt();
    assert!((norm - 1.0).abs() < 1e-12);
}

#[test]
fn test_dem_grid_bilinear_interpolation_contract() {
    let elevations = vec![10.0, 20.0, 30.0, 40.0];
    let grid = sightline_cot::camera::DemGrid::new(0.0, 0.0, 10.0, 2, 2, elevations).unwrap();
    assert_eq!(grid.elevation_at(0.0, 0.0), Some(10.0));
    assert_eq!(grid.elevation_at(10.0, 0.0), Some(20.0));
    assert_eq!(grid.elevation_at(0.0, 10.0), Some(30.0));
    assert_eq!(grid.elevation_at(10.0, 10.0), Some(40.0));
    let mid = grid.elevation_at(5.0, 5.0).unwrap();
    assert!((mid - 25.0).abs() < 1e-9);
}
