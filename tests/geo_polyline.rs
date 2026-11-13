//! Tests for polyline distance, intermediate points, and resampling.

use sightline_cot::geo_polyline::{
    great_circle_intermediate, haversine_distance, polyline_length,
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
