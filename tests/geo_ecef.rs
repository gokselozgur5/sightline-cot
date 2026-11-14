use sightline_cot::geo_ecef::{ecef_to_geodetic, geodetic_to_ecef};

#[test]
fn test_equator_prime_meridian_roundtrip() {
    let ecef = geodetic_to_ecef(0.0, 0.0, 0.0);
    assert!((ecef[0] - 6_378_137.0).abs() < 1e-3);
    assert!(ecef[1].abs() < 1e-3);
    assert!(ecef[2].abs() < 1e-3);

    let (lat, lon, hae) = ecef_to_geodetic(ecef[0], ecef[1], ecef[2]);
    assert!(lat.abs() < 1e-9);
    assert!(lon.abs() < 1e-9);
    assert!(hae.abs() < 1e-4);
}
