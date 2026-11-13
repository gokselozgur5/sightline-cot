//! Tests for the WGS84 meridian arc length.

use sightline_cot::geo_meridian_arc::meridian_arc;

#[test]
fn pole_to_equator_length() {
    let d = meridian_arc(0.0, 90.0);
    assert!((d - 10_001_965.729).abs() < 1e-3, "got {d}");
}

#[test]
fn antisymmetric_arguments() {
    for (a, b) in [(0.0, 45.0), (-30.0, 60.0), (10.0, 80.0)] {
        assert_eq!(meridian_arc(a, b), -meridian_arc(b, a));
    }
}

#[test]
fn degree_length_at_equator() {
    let d = meridian_arc(0.0, 1.0);
    assert!((d - 110_574.389).abs() < 1e-2, "got {d}");
}

#[test]
fn arcs_add_up() {
    let sum = meridian_arc(0.0, 30.0) + meridian_arc(30.0, 75.0);
    assert!((sum - meridian_arc(0.0, 75.0)).abs() < 1e-6);
}
