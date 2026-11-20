use sightline_cot::sensor_frustum::{
    compute_frustum_ground_enu, PlatformPose, SensorFrustumConfig, SensorGimbal,
};

#[test]
fn nose_up_pitch_does_not_lengthen_an_aft_looking_boresight() {
    let gimbal = SensorGimbal::new(180.0, -45.0, 0.0).unwrap();
    let cfg = SensorFrustumConfig::new(10.0, 10.0, 50_000.0, 1.0).unwrap();
    let level = PlatformPose::new(0.0, 0.0, 1000.0, 0.0, 0.0, 0.0).unwrap();
    let nose_up = PlatformPose::new(0.0, 0.0, 1000.0, 0.0, 10.0, 0.0).unwrap();
    let (b0, _, _) = compute_frustum_ground_enu(&level, &gimbal, &cfg, 0.0).unwrap();
    let (b1, _, _) = compute_frustum_ground_enu(&nose_up, &gimbal, &cfg, 0.0).unwrap();
    assert!((b0[1] + 1000.0).abs() < 1e-6, "level north: {}", b0[1]);
    assert!(b1[1].abs() <= 1000.0 + 1e-6, "nose-up north: {}", b1[1]);
    assert!((b1[1].abs() - 700.2).abs() < 0.1, "ground range: {}", b1[1]);
}
