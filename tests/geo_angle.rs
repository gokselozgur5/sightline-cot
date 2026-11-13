//! Tests for angle normalization helpers.

use sightline_cot::geo_angle::{angle_diff, wrap_180, wrap_360};

#[test]
fn wraps_to_360() {
    assert_eq!(wrap_360(-1.0), 359.0);
    assert_eq!(wrap_360(360.0), 0.0);
}

#[test]
fn wraps_to_180() {
    assert_eq!(wrap_180(190.0), -170.0);
    assert_eq!(wrap_180(180.0), 180.0);
}

#[test]
fn signed_difference_takes_short_way() {
    assert_eq!(angle_diff(350.0, 10.0), 20.0);
    assert_eq!(angle_diff(10.0, 350.0), -20.0);
}

#[test]
fn huge_angles_stay_finite() {
    for v in [1e9, -1e9] {
        assert!((0.0..360.0).contains(&wrap_360(v)));
        assert!(wrap_180(v).is_finite() && angle_diff(v, 0.0).is_finite());
    }
}
