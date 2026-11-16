use sightline_cot::cot_circle::{CotCircle, CotCircleError, CotRangeRings};

#[test]
fn test_circle_creation_and_error_validation() {
    let circle = CotCircle::new(34.0522, -118.2437, 500.0).expect("valid circle");
    assert_eq!(circle.lat, 34.0522);
    assert_eq!(circle.lon, -118.2437);
    assert_eq!(circle.radius_m, 500.0);

    assert!(matches!(CotCircle::new(95.0, 0.0, 100.0), Err(CotCircleError::InvalidCoordinate(_))));
    assert!(matches!(CotCircle::new(0.0, -190.0, 100.0), Err(CotCircleError::InvalidCoordinate(_))));
    assert!(matches!(CotCircle::new(0.0, 0.0, -10.0), Err(CotCircleError::InvalidRadius(_))));
    assert!(matches!(CotCircle::new(0.0, 0.0, 0.0), Err(CotCircleError::InvalidRadius(_))));
    assert!(matches!(circle.clone().with_stroke_weight(0.0), Err(CotCircleError::InvalidParameter(_))));
    assert!(matches!(circle.with_hae(f64::NAN), Err(CotCircleError::InvalidParameter(_))));
}

#[test]
fn test_circle_geometry_and_containment() {
    let circle = CotCircle::new(0.0, 0.0, 1000.0).expect("valid circle");
    let expected_area = std::f64::consts::PI * 1_000_000.0;
    assert!((circle.area_m2() - expected_area).abs() < 1e-6);
    let expected_circ = 2.0 * std::f64::consts::PI * 1000.0;
    assert!((circle.circumference_m() - expected_circ).abs() < 1e-6);

    assert!(circle.contains_point(0.0, 0.005).unwrap());
    assert!(!circle.contains_point(0.0, 0.05).unwrap());
}

#[test]
fn test_circle_to_polygon_vertices() {
    let circle = CotCircle::new(45.0, 10.0, 500.0).expect("valid circle");
    assert!(circle.to_polygon_vertices(2).is_err());
    let vertices = circle.to_polygon_vertices(8).expect("8 vertices");
    assert_eq!(vertices.len(), 8);
    for pt in vertices {
        let dist = sightline_cot::geo_haversine::haversine_distance_m([45.0, 10.0], pt);
        assert!((dist - 500.0).abs() < 1.0);
    }
}
