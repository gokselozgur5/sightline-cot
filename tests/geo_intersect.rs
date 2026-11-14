//! Tests for polyline self-intersection detector.

use sightline_cot::geo_intersect::{polyline_has_self_intersection, segments_intersect};

#[test]
fn test_perpendicular_segments_intersect() {
    let p1 = [0.0, 0.0];
    let p2 = [1.0, 1.0];
    let p3 = [0.0, 1.0];
    let p4 = [1.0, 0.0];
    let pt = segments_intersect(p1, p2, p3, p4);
    assert!(pt.is_some());
    let pt = pt.unwrap();
    assert!((pt[0] - 0.5).abs() < 1e-9);
    assert!((pt[1] - 0.5).abs() < 1e-9);
}

#[test]
fn test_parallel_segments_no_intersect() {
    let p1 = [0.0, 0.0];
    let p2 = [0.0, 1.0];
    let p3 = [1.0, 0.0];
    let p4 = [1.0, 1.0];
    assert!(segments_intersect(p1, p2, p3, p4).is_none());
}

use sightline_cot::geo_intersect::find_self_intersections;

#[test]
fn test_bowtie_polyline_self_intersection() {
    let polyline = [[0.0, 0.0], [1.0, 1.0], [0.0, 1.0], [1.0, 0.0]];
    assert!(polyline_has_self_intersection(&polyline));
    let hits = find_self_intersections(&polyline);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].segment_a, 0);
    assert_eq!(hits[0].segment_b, 2);
    assert!((hits[0].point[0] - 0.5).abs() < 1e-9);
    assert!((hits[0].point[1] - 0.5).abs() < 1e-9);
}

#[test]
fn test_closed_square_no_self_intersection() {
    let square = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];
    assert!(!polyline_has_self_intersection(&square));
    assert!(find_self_intersections(&square).is_empty());
}
