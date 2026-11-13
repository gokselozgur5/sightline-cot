use sightline_cot::geo_dest::{destination_point, haversine_distance, initial_bearing};

#[test]
fn test_destination_acceptance_roundtrip_range() {
    let lat = 37.7749;
    let lon = -122.4194;
    let bearing = 60.0;
    let range = 25_000.0;
    let (dest_lat, dest_lon) = destination_point(lat, lon, bearing, range).expect("destination");
    let back_dist = haversine_distance(lat, lon, dest_lat, dest_lon);
    let rel_err = (back_dist - range).abs() / range;
    assert!(rel_err < 1e-4, "relative error {} >= 1e-4", rel_err);
    let init_b = initial_bearing(lat, lon, dest_lat, dest_lon);
    assert!((init_b - bearing).abs() < 1e-4);
}
