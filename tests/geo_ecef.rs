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

#[test]
fn test_poles_and_singularity() {
    let north = geodetic_to_ecef(90.0, 0.0, 0.0);
    assert!(north[0].abs() < 1e-3 && north[1].abs() < 1e-3);
    assert!((north[2] - 6_356_752.3142).abs() < 1e-3);

    let (lat_n, _, hae_n) = ecef_to_geodetic(north[0], north[1], north[2]);
    assert!((lat_n - 90.0).abs() < 1e-9 && hae_n.abs() < 1e-4);

    let south = geodetic_to_ecef(-90.0, 45.0, 100.0);
    let (lat_s, _, hae_s) = ecef_to_geodetic(south[0], south[1], south[2]);
    assert!((lat_s + 90.0).abs() < 1e-9 && (hae_s - 100.0).abs() < 1e-4);
}
