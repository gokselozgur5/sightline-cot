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
