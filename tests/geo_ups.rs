use sightline_cot::geo_ups::{UpsCoord, UpsHemisphere};

#[test]
fn test_ups_coord_types() {
    let coord = UpsCoord::new(2000000.0, 2000000.0, UpsHemisphere::North);
    assert_eq!(coord.easting, 2000000.0);
    assert_eq!(coord.northing, 2000000.0);
    assert_eq!(coord.hemisphere, UpsHemisphere::North);
}

use sightline_cot::geo_ups::wgs84_to_ups;

#[test]
fn test_north_pole_forward() {
    let ups = wgs84_to_ups(90.0, 0.0).unwrap();
    assert_eq!(ups.hemisphere, UpsHemisphere::North);
    assert!((ups.easting - 2000000.0).abs() < 1e-4);
    assert!((ups.northing - 2000000.0).abs() < 1e-4);
}


#[test]
fn test_south_pole_forward() {
    let ups = wgs84_to_ups(-90.0, 0.0).unwrap();
    assert_eq!(ups.hemisphere, UpsHemisphere::South);
    assert!((ups.easting - 2000000.0).abs() < 1e-4);
    assert!((ups.northing - 2000000.0).abs() < 1e-4);
}

use sightline_cot::geo_ups::ups_to_wgs84;

#[test]
fn test_ups_roundtrip_north() {
    let (orig_lat, orig_lon) = (85.0, 45.0);
    let ups = wgs84_to_ups(orig_lat, orig_lon).unwrap();
    let (lat, lon) = ups_to_wgs84(ups).unwrap();
    assert!((lat - orig_lat).abs() < 1e-6);
    assert!((lon - orig_lon).abs() < 1e-6);
}

use sightline_cot::geo_ups::UpsError;

#[test]
fn test_ups_validation_and_errors() {
    assert_eq!(wgs84_to_ups(45.0, 0.0), Err(UpsError::InvalidLatitude));
    assert_eq!(wgs84_to_ups(f64::NAN, 0.0), Err(UpsError::NonFinite));
    assert_eq!(ups_to_wgs84(UpsCoord::new(f64::NAN, 0.0, UpsHemisphere::North)), Err(UpsError::NonFinite));
}


#[test]
fn test_ups_root_exports() {
    use sightline_cot::{wgs84_to_ups, ups_to_wgs84};
    let ups = wgs84_to_ups(89.0, 10.0).unwrap();
    let (lat, lon) = ups_to_wgs84(ups).unwrap();
    assert!((lat - 89.0).abs() < 1e-5);
    assert!((lon - 10.0).abs() < 1e-5);
}
