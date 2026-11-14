use sightline_cot::geo_destination::destination_point;
use sightline_cot::geo_greatcircle::haversine_distance;

#[test]
fn test_zero_distance_and_cardinals() {
    let (la, lo) = destination_point(10.0, 20.0, 77.0, 0.0);
    assert!((la - 10.0).abs() < 1e-9 && (lo - 20.0).abs() < 1e-9);
    let (la, _) = destination_point(0.0, 0.0, 0.0, 111_195.0);
    assert!((la - 1.0).abs() < 1e-3);
    let (_, lo) = destination_point(0.0, 0.0, 90.0, 111_195.0);
    assert!((lo - 1.0).abs() < 1e-3);
}

#[test]
fn test_roundtrip_and_antimeridian() {
    for (lat, lon, b) in [(37.7, -122.4, 45.0), (0.0, 179.9, 90.0), (-30.0, -179.9, 270.0)] {
        let d = 50_000.0;
        let (la, lo) = destination_point(lat, lon, b, d);
        assert!((haversine_distance(lat, lon, la, lo) - d).abs() / d < 1e-6);
        assert!((-180.0..=180.0).contains(&lo));
    }
}
