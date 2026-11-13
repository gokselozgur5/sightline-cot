//! Tests for rhumb line distance, bearing, and destination.

use sightline_cot::geo_rhumb::{rhumb_bearing, rhumb_distance, WGS84_EARTH_RADIUS};

#[test]
fn test_due_east_rhumb_distance_equals_cos_lat_times_equatorial_arc() {
    let lat = 45.0_f64;
    let p1 = [lat, 10.0];
    let p2 = [lat, 20.0];
    let d = rhumb_distance(p1, p2);
    let equatorial_arc = WGS84_EARTH_RADIUS * 10.0_f64.to_radians();
    let expected = lat.to_radians().cos() * equatorial_arc;
    assert!((d - expected).abs() < 1e-6, "got {d} vs {expected}");
}

#[test]
fn test_antimeridian_crossing() {
    let p1 = [0.0, 179.0];
    let p2 = [0.0, -179.0];
    let d = rhumb_distance(p1, p2);
    let expected = WGS84_EARTH_RADIUS * 2.0_f64.to_radians();
    assert!((d - expected).abs() < 1e-6, "got {d} vs {expected}");
    assert!((rhumb_bearing(p1, p2) - 90.0).abs() < 1e-6);
}
