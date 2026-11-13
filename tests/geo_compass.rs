//! Tests for compass point names.

use sightline_cot::geo_compass::compass_name;

#[test]
fn sixteen_point_names() {
    assert_eq!(compass_name(0.0, 16), Some("N"));
    assert_eq!(compass_name(22.5, 16), Some("NNE"));
    assert_eq!(compass_name(90.0, 16), Some("E"));
    assert_eq!(compass_name(337.4, 16), Some("NNW"));
}

#[test]
fn eight_point_names() {
    assert_eq!(compass_name(44.0, 8), Some("NE"));
    assert_eq!(compass_name(200.0, 8), Some("S"));
    assert_eq!(compass_name(315.0, 8), Some("NW"));
}

#[test]
fn wraps_and_rejects_bad_input() {
    assert_eq!(compass_name(360.0, 16), Some("N"));
    assert_eq!(compass_name(-90.0, 8), Some("W"));
    assert_eq!(compass_name(359.9, 8), Some("N"));
    assert_eq!(compass_name(f64::NAN, 8), None);
    assert_eq!(compass_name(10.0, 12), None);
}
