use sightline_cot::sensor_frustum::{
    compute_frustum_ground_enu, generate_sensor_footprint, PlatformPose, SensorFrustumConfig,
    SensorGimbal,
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

#[test]
fn nadir_footprint_centroid_stays_under_the_platform_at_the_antimeridian() {
    let platform = PlatformPose::new(0.0, 179.999, 1000.0, 0.0, 0.0, 0.0).unwrap();
    let gimbal = SensorGimbal::new(0.0, -90.0, 0.0).unwrap();
    let cfg = SensorFrustumConfig::new(60.0, 60.0, 5000.0, 10.0).unwrap();
    let fp = generate_sensor_footprint(&platform, &gimbal, &cfg, 0.0).unwrap();
    let lon = fp.centroid_geo()[1];
    let diff = (lon - 179.999).rem_euclid(360.0);
    let sep_deg = diff.min(360.0 - diff);
    assert!(
        sep_deg < 0.01,
        "centroid lon {lon} is {sep_deg} degrees from the platform"
    );
}

#[test]
fn parse_cot_sensor_reads_the_exact_attribute_names() {
    let xml = r#"<sensor azimuth="10" elevation="-30" vfov="20" fov="40" range="1000"/>"#;
    let (gimbal, fov, range) = SensorGimbal::parse_cot_sensor(xml).unwrap();
    assert_eq!(gimbal.azimuth_deg, 10.0);
    assert_eq!(fov, 40.0, "fov must not be read from vfov");
    assert_eq!(range, 1000.0);
}

#[test]
fn parse_cot_sensor_rejects_non_finite_fov_and_range() {
    let xml = r#"<sensor azimuth="10" elevation="-30" fov="inf" range="NaN"/>"#;
    assert!(SensorGimbal::parse_cot_sensor(xml).is_err());
}

#[test]
fn oblique_footprint_is_clipped_at_max_range_and_bounded_by_min_range() {
    let platform = PlatformPose::new(0.0, 0.0, 1000.0, 0.0, 0.0, 0.0).unwrap();
    let gimbal = SensorGimbal::new(0.0, -10.0, 0.0).unwrap();
    let cfg = SensorFrustumConfig::new(30.0, 30.0, 20_000.0, 10.0).unwrap();
    let fp = generate_sensor_footprint(&platform, &gimbal, &cfg, 0.0).unwrap();
    assert_eq!(fp.vertices_enu.len(), 4);
    assert!((fp.slant_range_m - 5758.77).abs() < 1.0);
    let cfg_min = SensorFrustumConfig::new(30.0, 30.0, 20_000.0, 3000.0).unwrap();
    let fp_min = generate_sensor_footprint(&platform, &gimbal, &cfg_min, 0.0).unwrap();
    assert!(fp_min.area_sq_m < fp.area_sq_m);
}

#[test]
fn test_oblique_footprint_pins_corner_positions_and_orientation() {
    let platform = PlatformPose::new(0.0, 0.0, 500.0, 0.0, 0.0, 0.0).unwrap();
    let gimbal = SensorGimbal::new(0.0, -45.0, 0.0).unwrap();
    let cfg = SensorFrustumConfig::new(30.0, 30.0, 2000.0, 5.0).unwrap();
    let fp = generate_sensor_footprint(&platform, &gimbal, &cfg, 0.0).unwrap();
    assert_eq!(fp.vertices_enu.len(), 4);
    assert!((fp.vertices_enu[0][0] - (-149.4)).abs() < 1.0);
    assert!((fp.vertices_enu[0][1] - 288.7).abs() < 1.0);
    assert!((fp.vertices_enu[1][0] - 149.4).abs() < 1.0);
    assert!((fp.vertices_enu[1][1] - 288.7).abs() < 1.0);
    assert!((fp.vertices_enu[2][0] - 258.8).abs() < 1.0);
    assert!((fp.vertices_enu[2][1] - 866.0).abs() < 1.0);
    assert!((fp.vertices_enu[3][0] - (-258.8)).abs() < 1.0);
    assert!((fp.vertices_enu[3][1] - 866.0).abs() < 1.0);
}
