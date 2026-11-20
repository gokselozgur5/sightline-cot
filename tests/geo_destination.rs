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
    for (lat, lon, b) in [
        (37.7, -122.4, 45.0),
        (0.0, 179.9, 90.0),
        (-30.0, -179.9, 270.0),
    ] {
        let d = 50_000.0;
        let (la, lo) = destination_point(lat, lon, b, d);
        assert!((haversine_distance(lat, lon, la, lo) - d).abs() / d < 1e-6);
        assert!((-180.0..=180.0).contains(&lo));
    }
}

#[test]
fn test_polar_departure_bearing_longitude() {
    let (_, lo_n) = destination_point(90.0, 0.0, 135.0, 100_000.0);
    assert!((lo_n - 45.0).abs() < 1e-6);
    let (_, lo_s) = destination_point(-90.0, 0.0, 45.0, 100_000.0);
    assert!((lo_s - 45.0).abs() < 1e-6);
}

#[test]
fn test_destination_point_with_custom_radius() {
    use sightline_cot::geo_destination::destination_point_with_radius;
    let (la, _) = destination_point_with_radius(0.0, 0.0, 0.0, 1_000.0, 1_000.0);
    assert!((la - 1.0_f64.to_degrees()).abs() < 1e-6);
}

#[test]
fn test_final_bearing_equator_and_cardinals() {
    use sightline_cot::geo_destination::final_bearing;
    let b_east = final_bearing(0.0, 0.0, 90.0, 100_000.0);
    assert!((b_east - 90.0).abs() < 1e-6);
    let b_north = final_bearing(0.0, 0.0, 0.0, 100_000.0);
    assert!((b_north - 0.0).abs() < 1e-6);
}

#[test]
fn test_final_bearing_matches_reverse_initial_bearing() {
    use sightline_cot::geo_destination::final_bearing;
    use sightline_cot::geo_greatcircle::initial_bearing;
    let (lat1, lon1, b1, d) = (37.7, -122.4, 45.0, 500_000.0);
    let (lat2, lon2) = destination_point(lat1, lon1, b1, d);
    let fb = final_bearing(lat1, lon1, b1, d);
    let rev_b = (initial_bearing(lat2, lon2, lat1, lon1) + 180.0) % 360.0;
    assert!((fb - rev_b).abs() < 1e-4);
}
