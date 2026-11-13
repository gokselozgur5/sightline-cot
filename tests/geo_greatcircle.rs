use sightline_cot::geo_greatcircle::{haversine_distance, EARTH_RADIUS_M};

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
fn test_haversine_antimeridian() {
    let d = haversine_distance(0.0, 179.0, 0.0, -179.0);
    let expected = 2.0 * std::f64::consts::PI / 180.0 * EARTH_RADIUS_M;
    assert!((d - expected).abs() < 1.0);
}
