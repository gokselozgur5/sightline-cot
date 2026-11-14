use sightline_cot::geo_frame::LocalFrame;

#[test]
fn test_geo_frame_cached_trig_and_projection() {
    let frame = LocalFrame::new(37.7749, -122.4194, 10.0).unwrap();
    assert!((frame.sin_lat - 37.7749_f64.to_radians().sin()).abs() < 1e-12);
    assert!((frame.cos_lat - 37.7749_f64.to_radians().cos()).abs() < 1e-12);
    let enu = frame.geodetic_to_enu(37.7749, -122.4194, 10.0).unwrap();
    assert!(enu[0].abs() < 1e-6);
    assert!(enu[1].abs() < 1e-6);
    assert!(enu[2].abs() < 1e-6);
}
