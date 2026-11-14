use sightline_cot::cam_slant_range::{
    compute_slant_range_and_grazing, slant_range_for_pixel, slant_range_from_pose_and_ground,
};
use sightline_cot::camera::{CameraError, CameraPose, Intrinsics};
use std::f64::consts::PI;

#[test]
fn test_nadir_slant_range_and_grazing() {
    let sensor = [0.0, 0.0, 100.0];
    let target = [0.0, 0.0, 0.0];
    let geom = compute_slant_range_and_grazing(sensor, target).unwrap();
    assert!((geom.slant_range_m - 100.0).abs() < 1e-9);
    assert!((geom.ground_range_m - 0.0).abs() < 1e-9);
    assert!((geom.grazing_angle_deg() - 90.0).abs() < 1e-9);
    assert!((geom.elevation_angle_deg() - (-90.0)).abs() < 1e-9);
}

#[test]
fn test_oblique_45_deg_geometry() {
    let sensor = [0.0, 0.0, 100.0];
    let target = [60.0, 80.0, 0.0];
    let geom = compute_slant_range_and_grazing(sensor, target).unwrap();
    assert!((geom.ground_range_m - 100.0).abs() < 1e-9);
    assert!((geom.slant_range_m - (20000.0_f64).sqrt()).abs() < 1e-9);
    assert!((geom.grazing_angle_deg() - 45.0).abs() < 1e-9);
    assert!((geom.elevation_angle_deg() - (-45.0)).abs() < 1e-9);
}

#[test]
fn test_slant_range_from_pose_and_ground() {
    let pose = CameraPose {
        roll_rad: 0.0,
        pitch_rad: PI / 4.0,
        yaw_rad: 0.0,
        cam_up_m: 100.0,
    };
    let ground_pt = [0.0, 100.0, 0.0];
    let geom = slant_range_from_pose_and_ground(&pose, ground_pt).unwrap();
    assert!((geom.ground_range_m - 100.0).abs() < 1e-9);
    assert!((geom.slant_range_m - 141.421356237).abs() < 1e-6);
    assert!((geom.grazing_angle_deg() - 45.0).abs() < 1e-9);
}

#[test]
fn test_slant_range_for_pixel_end_to_end() {
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
    let geom = slant_range_for_pixel(&cam, &pose, 500.0, 500.0, 0.0).unwrap();
    assert!((geom.slant_range_m - 100.0).abs() < 1e-6);
    assert!((geom.grazing_angle_deg() - 90.0).abs() < 1e-6);
}

#[test]
fn test_slant_range_error_handling() {
    let sensor = [f64::NAN, 0.0, 100.0];
    let target = [0.0, 0.0, 0.0];
    assert_eq!(
        compute_slant_range_and_grazing(sensor, target),
        Err(CameraError::InvalidParameters)
    );

    let same_pos = [10.0, 20.0, 30.0];
    assert_eq!(
        compute_slant_range_and_grazing(same_pos, same_pos),
        Err(CameraError::InvalidParameters)
    );
}
