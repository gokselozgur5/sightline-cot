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

use sightline_cot::geo_rhumb::{rhumb_destination, rhumb_intermediate};

#[test]
fn test_rhumb_destination_and_intermediate_roundtrip() {
    let start = [10.0, 20.0];
    let bearing = 45.0;
    let dist = 100_000.0;
    let dest = rhumb_destination(start, bearing, dist);
    let measured_dist = rhumb_distance(start, dest);
    assert!((measured_dist - dist).abs() < 1e-4);
    let measured_bearing = rhumb_bearing(start, dest);
    assert!((measured_bearing - bearing).abs() < 1e-6);

    let mid = rhumb_intermediate(start, dest, 0.5);
    let d1 = rhumb_distance(start, mid);
    let d2 = rhumb_distance(mid, dest);
    assert!((d1 - dist / 2.0).abs() < 1e-4);
    assert!((d2 - dist / 2.0).abs() < 1e-4);
}

#[test]
fn test_rhumb_coincident_points_at_south_pole() {
    let p = [-90.0, 0.0];
    let d = rhumb_distance(p, p);
    assert!(!d.is_nan(), "distance is NaN");
    assert_eq!(d, 0.0);
    let b = rhumb_bearing(p, p);
    assert!(!b.is_nan(), "bearing is NaN");
    assert_eq!(b, 0.0);
}

#[test]
fn test_rhumb_polar_destination_and_intermediate() {
    let south_pole = [-90.0, 0.0];
    let dest = rhumb_destination(south_pole, 90.0, 1000.0);
    assert!(!dest[0].is_nan() && !dest[1].is_nan());
    assert!((dest[0] - (-90.0)).abs() < 1e-6);

    let mid = rhumb_intermediate(south_pole, [-90.0, 60.0], 0.5);
    assert!(!mid[0].is_nan() && !mid[1].is_nan());
    assert!((mid[0] - (-90.0)).abs() < 1e-6);
    assert!((mid[1] - 30.0).abs() < 1e-6);
}
