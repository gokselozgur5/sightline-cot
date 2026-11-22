use sightline_cot::geo_unified::{wgs84_to_unified, UnifiedCoord};
use sightline_cot::geo_ups::{ups_to_wgs84, wgs84_to_ups, UpsCoord};
use sightline_cot::GridError;

#[test]
fn test_ups_coord_types() {
    let coord = UpsCoord {
        hemisphere: 'N',
        easting: 2000000.0,
        northing: 2000000.0,
    };
    assert_eq!(coord.hemisphere, 'N');
    assert_eq!(coord.easting, 2000000.0);
    assert_eq!(coord.northing, 2000000.0);
}

#[test]
fn test_north_pole_forward() {
    let ups = wgs84_to_ups(90.0, 0.0).unwrap();
    assert_eq!(ups.hemisphere, 'N');
    assert!((ups.easting - 2000000.0).abs() < 1e-4);
    assert!((ups.northing - 2000000.0).abs() < 1e-4);
}

#[test]
fn test_south_pole_forward() {
    let ups = wgs84_to_ups(-90.0, 0.0).unwrap();
    assert_eq!(ups.hemisphere, 'S');
    assert!((ups.easting - 2000000.0).abs() < 1e-4);
    assert!((ups.northing - 2000000.0).abs() < 1e-4);
}

#[test]
fn test_ups_roundtrip_north() {
    let (orig_lat, orig_lon) = (85.0, 45.0);
    let ups = wgs84_to_ups(orig_lat, orig_lon).unwrap();
    let (lat, lon) = ups_to_wgs84(&ups).unwrap();
    assert!((lat - orig_lat).abs() < 1e-6);
    assert!((lon - orig_lon).abs() < 1e-6);
}

#[test]
fn test_ups_roundtrip_south() {
    let (orig_lat, orig_lon) = (-85.0, 45.0);
    let ups = wgs84_to_ups(orig_lat, orig_lon).unwrap();
    let (lat, lon) = ups_to_wgs84(&ups).unwrap();
    assert!((lat - orig_lat).abs() < 1e-6);
    assert!((lon - orig_lon).abs() < 1e-6);
}

#[test]
fn test_ups_validation_and_errors() {
    assert_eq!(wgs84_to_ups(45.0, 0.0), Err(GridError::InvalidLatitude));
    assert_eq!(wgs84_to_ups(f64::NAN, 0.0), Err(GridError::InvalidLatitude));
    assert_eq!(wgs84_to_ups(95.0, 0.0), Err(GridError::InvalidLatitude));
    assert_eq!(wgs84_to_ups(85.0, 190.0), Err(GridError::InvalidLongitude));
    let invalid_hemi = UpsCoord {
        hemisphere: 'X',
        easting: 2000000.0,
        northing: 2000000.0,
    };
    assert_eq!(ups_to_wgs84(&invalid_hemi), Err(GridError::InvalidLatitude));
    let invalid_easting = UpsCoord {
        hemisphere: 'N',
        easting: -100.0,
        northing: 2000000.0,
    };
    assert_eq!(
        ups_to_wgs84(&invalid_easting),
        Err(GridError::InvalidEasting)
    );
}

#[test]
fn test_ups_root_exports() {
    use sightline_cot::{ups_to_wgs84, wgs84_to_ups};
    let ups = wgs84_to_ups(89.0, 10.0).unwrap();
    let (lat, lon) = ups_to_wgs84(&ups).unwrap();
    assert!((lat - 89.0).abs() < 1e-5);
    assert!((lon - 10.0).abs() < 1e-5);
}

#[test]
fn test_ups_reference_scale() {
    let ups = wgs84_to_ups(85.0, 0.0).unwrap();
    assert!((ups.northing - 1444542.61).abs() < 1.0);
    assert!((ups.easting - 2000000.0).abs() < 1e-4);
}

#[test]
fn test_unified_to_ups_interoperability() {
    let unified = wgs84_to_unified(85.0, 45.0).unwrap();
    match unified {
        UnifiedCoord::Ups(ups) => {
            let (lat, lon) = sightline_cot::ups_to_wgs84(&ups).unwrap();
            assert!((lat - 85.0).abs() < 1e-6);
            assert!((lon - 45.0).abs() < 1e-6);
        }
        _ => panic!("expected UnifiedCoord::Ups"),
    }
}
