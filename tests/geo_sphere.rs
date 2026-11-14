use sightline_cot::geo_sphere::{haversine_distance, initial_bearing};

#[test]
fn test_haversine_identical_and_symmetric() {
    let d0 = haversine_distance(37.7749, -122.4194, 37.7749, -122.4194);
    assert!(d0.abs() < 1e-9);
    let d1 = haversine_distance(51.5074, -0.1278, 48.8566, 2.3522);
    let d2 = haversine_distance(48.8566, 2.3522, 51.5074, -0.1278);
    assert!((d1 - d2).abs() < 1e-9);
    assert!((d1 - 343_500.0).abs() < 2_000.0);
}

#[test]
fn test_initial_bearing_cardinal_directions() {
    assert_eq!(initial_bearing(0.0, 0.0, 0.0, 0.0), 0.0);
    assert!((initial_bearing(0.0, 0.0, 10.0, 0.0) - 0.0).abs() < 1e-6);
    assert!((initial_bearing(0.0, 0.0, 0.0, 10.0) - 90.0).abs() < 1e-6);
    assert!((initial_bearing(0.0, 0.0, -10.0, 0.0) - 180.0).abs() < 1e-6);
    assert!((initial_bearing(0.0, 0.0, 0.0, -10.0) - 270.0).abs() < 1e-6);
}

#[test]
fn test_haversine_antimeridian_and_antipodal() {
    use sightline_cot::geo_sphere::EARTH_RADIUS_M;
    let d = haversine_distance(0.0, 179.0, 0.0, -179.0);
    let expected = 2.0 * std::f64::consts::PI / 180.0 * EARTH_RADIUS_M;
    assert!((d - expected).abs() < 1.0);
    let d_pole = haversine_distance(90.0, 0.0, -90.0, 0.0);
    assert!((d_pole - std::f64::consts::PI * EARTH_RADIUS_M).abs() < 1.0);
}
