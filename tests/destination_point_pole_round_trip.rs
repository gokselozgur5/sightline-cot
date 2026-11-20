use sightline_cot::geo_destination::destination_point;
use sightline_cot::geo_greatcircle::{haversine_distance, initial_bearing};

fn round_trip_error_deg(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let b = initial_bearing(lat1, lon1, lat2, lon2);
    let d = haversine_distance(lat1, lon1, lat2, lon2);
    let (lat, lon) = destination_point(lat1, lon1, b, d);
    let dlon = (lon - lon2 + 540.0).rem_euclid(360.0) - 180.0;
    (lat - lat2).abs().max(dlon.abs())
}

#[test]
fn round_trip_from_near_pole_start_reaches_target() {
    assert!(round_trip_error_deg(89.999, 77.0, 81.0, 20.0) < 1e-6);
    assert!(round_trip_error_deg(-89.999, 77.0, -81.0, 20.0) < 1e-6);
}

#[test]
fn round_trip_from_north_pole_start_reaches_target() {
    let err = round_trip_error_deg(90.0, 77.0, 81.0, 20.0);
    assert!(err < 1e-6, "north pole round trip error {err} deg");
}

#[test]
fn round_trip_from_south_pole_start_reaches_target() {
    let err = round_trip_error_deg(-90.0, 77.0, -81.0, 20.0);
    assert!(err < 1e-6, "south pole round trip error {err} deg");
}
