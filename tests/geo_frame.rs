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

#[test]
fn test_geo_frame_error_handling() {
    use sightline_cot::geo_frame::GeoFrameError;

    assert_eq!(LocalFrame::new(f64::NAN, 0.0, 0.0), Err(GeoFrameError::NonFinite));
    assert_eq!(LocalFrame::new(95.0, 0.0, 0.0), Err(GeoFrameError::LatitudeOutOfRange));
    let frame = LocalFrame::new(0.0, 0.0, 0.0).unwrap();
    assert_eq!(frame.geodetic_to_enu(f64::NAN, 0.0, 0.0), Err(GeoFrameError::NonFinite));
    assert_eq!(frame.geodetic_to_enu(95.0, 0.0, 0.0), Err(GeoFrameError::LatitudeOutOfRange));
    assert_eq!(frame.enu_to_geodetic([f64::NAN, 0.0, 0.0]), Err(GeoFrameError::NonFinite));
}

#[test]
fn test_geo_frame_batch_roundtrip() {
    let frame = LocalFrame::new(45.0, 9.0, 100.0).unwrap();
    let coords = [
        [45.0, 9.0, 100.0],
        [45.01, 9.02, 150.0],
        [44.99, 8.98, 50.0],
    ];
    let enu_batch = frame.geodetic_to_enu_batch(&coords).unwrap();
    let back = frame.enu_to_geodetic_batch(&enu_batch).unwrap();
    for (orig, res) in coords.iter().zip(back.iter()) {
        assert!((orig[0] - res.0).abs() < 1e-7);
        assert!((orig[1] - res.1).abs() < 1e-7);
        assert!((orig[2] - res.2).abs() < 1e-3);
    }
}
