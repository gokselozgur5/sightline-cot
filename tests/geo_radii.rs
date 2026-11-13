//! Tests for WGS84 radii of curvature.

use sightline_cot::geo_radii::{meridian_radius, prime_vertical_radius};

#[test]
fn equator_values() {
    assert!((meridian_radius(0.0) - 6_335_439.327).abs() < 1e-3);
    assert!((prime_vertical_radius(0.0) - 6_378_137.0).abs() < 1e-3);
}

#[test]
fn polar_values() {
    let c = 6_399_593.626;
    assert!((meridian_radius(90.0) - c).abs() < 1e-2);
    assert!((prime_vertical_radius(90.0) - c).abs() < 1e-2);
}

#[test]
fn symmetric_in_latitude() {
    for lat in [10.0, 45.0, 80.0] {
        assert_eq!(meridian_radius(lat), meridian_radius(-lat));
        assert!(prime_vertical_radius(lat) >= meridian_radius(lat));
    }
}
