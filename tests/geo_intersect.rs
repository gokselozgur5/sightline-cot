//! Tests for polyline self-intersection detector.

use sightline_cot::geo_intersect::polyline_has_self_intersection;

#[test]
fn test_bowtie_polyline_self_intersection() {
    let polyline = [[0.0, 0.0], [1.0, 1.0], [0.0, 1.0], [1.0, 0.0]];
    assert!(polyline_has_self_intersection(&polyline));
}
