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
