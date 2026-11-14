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

#[test]
fn test_geo_frame_2d_and_enu_consistency() {
    use sightline_cot::geo_enu::geodetic_to_enu;

    let frame = LocalFrame::new(37.77, -122.41, 50.0).unwrap();
    let pts = [[37.78, -122.40], [37.76, -122.42]];
    let proj = frame.project_2d_batch(&pts).unwrap();
    for (pt, p2d) in pts.iter().zip(proj.iter()) {
        let enu_ref = geodetic_to_enu(pt[0], pt[1], 0.0, 37.77, -122.41, 50.0).unwrap();
        assert!((p2d[0] - enu_ref[0]).abs() < 1e-6);
        assert!((p2d[1] - enu_ref[1]).abs() < 1e-6);
        let unp = frame.unproject_2d(p2d[0], p2d[1]).unwrap();
        assert!((unp.0 - pt[0]).abs() < 1e-7);
        assert!((unp.1 - pt[1]).abs() < 1e-7);
    }
}

#[test]
fn test_geo_frame_ecef_roundtrip() {
    let frame = LocalFrame::new(-45.0, 170.0, 200.0).unwrap();
    let offsets = [[0.0, 0.0, 0.0], [500.0, -300.0, 50.0], [-1000.0, 2000.0, -100.0]];
    for &enu in &offsets {
        let ecef = frame.enu_to_ecef(enu).unwrap();
        let enu_back = frame.ecef_to_enu(ecef).unwrap();
        assert!((enu[0] - enu_back[0]).abs() < 1e-9);
        assert!((enu[1] - enu_back[1]).abs() < 1e-9);
        assert!((enu[2] - enu_back[2]).abs() < 1e-9);
    }
}
