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

#[test]
fn test_circle_xml_roundtrip() {
    let circle = CotCircle::new(37.7749, -122.4194, 750.0).expect("valid circle")
        .with_callsign("PERIMETER_1")
        .with_stroke_color(-65536)
        .with_stroke_weight(2.5).expect("weight")
        .with_remarks("Test Circle");
    let xml = circle.to_cot_event_xml("UID-C1", 1700000000000, 1700000060000);
    assert!(xml.contains("major=\"750.00\""));
    assert!(xml.contains("minor=\"750.00\""));
    let parsed = CotCircle::parse(&xml).expect("parse xml");
    assert_eq!(parsed.radius_m, 750.0);
    assert_eq!(parsed.callsign.as_deref(), Some("PERIMETER_1"));
    assert_eq!(parsed.stroke_color, Some(-65536));
}

#[test]
fn test_range_rings_generation_and_lookup() {
    let rings = CotRangeRings::from_interval(34.0, -118.0, 500.0, 3).expect("rings")
        .with_callsign_prefix("AIRPORT");
    assert_eq!(rings.radii_m, vec![500.0, 1000.0, 1500.0]);
    assert_eq!(rings.ring_index_for_distance(250.0), Some(0));
    assert_eq!(rings.ring_index_for_distance(750.0), Some(1));
    assert_eq!(rings.ring_index_for_distance(1250.0), Some(2));
    assert_eq!(rings.ring_index_for_distance(2000.0), None);

    let events = rings.to_cot_events("BASE", 1700000000000, 1700000060000);
    assert_eq!(events.len(), 3);
    assert!(events[0].contains("BASE-ring-1"));
    assert!(events[1].contains("AIRPORT Ring 2"));
}
