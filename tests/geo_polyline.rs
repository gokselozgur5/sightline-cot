//! Tests for polyline distance, intermediate points, and resampling.

use sightline_cot::geo_polyline::{
    great_circle_intermediate, haversine_distance, polyline_length, polyline_resample,
    WGS84_EARTH_RADIUS,
};

#[test]
fn haversine_equator_degree() {
    let p1 = [0.0, 0.0];
    let p2 = [0.0, 1.0];
    let d = haversine_distance(p1, p2);
    assert!((d - 111_195.0).abs() < 200.0, "got {d}");
}

#[test]
fn great_circle_midpoint() {
    let p1 = [0.0, 0.0];
    let p2 = [0.0, 10.0];
    let mid = great_circle_intermediate(p1, p2, 0.5);
    assert!((mid[0] - 0.0).abs() < 1e-9);
    assert!((mid[1] - 5.0).abs() < 1e-9);
}

#[test]
fn polyline_length_empty_and_single() {
    assert_eq!(polyline_length(&[]), 0.0);
    assert_eq!(polyline_length(&[[10.0, 20.0]]), 0.0);
}

#[test]
fn resample_1km_at_100m_acceptance() {
    let delta = 1000.0 / WGS84_EARTH_RADIUS;
    let line = [[0.0, 0.0], [0.0, delta.to_degrees()]];
    assert!((polyline_length(&line) - 1000.0).abs() < 1e-6);
    let pts = polyline_resample(&line, 100.0);
    assert_eq!(pts.len(), 11);
    for i in 0..10 {
        let d = haversine_distance(pts[i], pts[i + 1]);
        let rel_err = (d - 100.0).abs() / 100.0;
        assert!(rel_err < 1e-6, "rel error {rel_err} at {i}");
    }
}
