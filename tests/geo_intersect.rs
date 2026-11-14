//! Tests for polyline self-intersection detector.

use sightline_cot::geo_intersect::polyline_has_self_intersection;

#[test]
fn test_bowtie_polyline_self_intersection() {
    let polyline = [[0.0, 0.0], [1.0, 1.0], [0.0, 1.0], [1.0, 0.0]];
    assert!(polyline_has_self_intersection(&polyline));
}

use sightline_cot::geo_intersect::segments_intersect;

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
