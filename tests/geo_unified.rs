use sightline_cot::geo_unified::{unified_to_wgs84, wgs84_to_unified, GridSystem};

#[test]
fn test_unified_utm_roundtrip() {
    let lat = 37.7749;
    let lon = -122.4194;
    let coord = wgs84_to_unified(lat, lon).expect("to unified");
    assert_eq!(coord.grid_system(), GridSystem::Utm);
    assert_eq!(coord.hemisphere(), 'N');
    assert_eq!(coord.zone(), Some(10));
    let (lat_out, lon_out) = unified_to_wgs84(&coord).expect("unproject");
    assert!((lat - lat_out).abs() < 1e-4);
    assert!((lon - lon_out).abs() < 1e-4);
}

#[test]
fn test_unified_ups_north_roundtrip() {
    let lat = 86.25;
    let lon = 120.0;
    let coord = wgs84_to_unified(lat, lon).expect("to unified north");
    assert_eq!(coord.grid_system(), GridSystem::Ups);
    assert_eq!(coord.hemisphere(), 'N');
    assert_eq!(coord.zone(), None);
    let (lat_out, lon_out) = unified_to_wgs84(&coord).expect("unproject");
    assert!((lat - lat_out).abs() < 1e-6);
    assert!((lon - lon_out).abs() < 1e-6);
}

#[test]
fn test_unified_ups_south_roundtrip() {
    let lat = -82.5;
    let lon = -70.0;
    let coord = wgs84_to_unified(lat, lon).expect("to unified south");
    assert_eq!(coord.grid_system(), GridSystem::Ups);
    assert_eq!(coord.hemisphere(), 'S');
    assert_eq!(coord.zone(), None);
    let (lat_out, lon_out) = unified_to_wgs84(&coord).expect("unproject");
    assert!((lat - lat_out).abs() < 1e-6);
    assert!((lon - lon_out).abs() < 1e-6);
}

#[test]
fn test_unified_exact_poles() {
    let north = wgs84_to_unified(90.0, 0.0).expect("exact north pole");
    assert!((north.easting() - 2000000.0).abs() < 1e-4);
    assert!((north.northing() - 2000000.0).abs() < 1e-4);
    let (lat_n, _) = unified_to_wgs84(&north).expect("unproject north pole");
    assert!((lat_n - 90.0).abs() < 1e-6);

    let south = wgs84_to_unified(-90.0, 0.0).expect("exact south pole");
    assert!((south.easting() - 2000000.0).abs() < 1e-4);
    assert!((south.northing() - 2000000.0).abs() < 1e-4);
    let (lat_s, _) = unified_to_wgs84(&south).expect("unproject south pole");
    assert!((lat_s - (-90.0)).abs() < 1e-6);
}

#[test]
fn test_unified_out_of_bounds() {
    use sightline_cot::GridError;
    assert_eq!(wgs84_to_unified(91.0, 0.0), Err(GridError::InvalidLatitude));
    assert_eq!(
        wgs84_to_unified(-91.0, 0.0),
        Err(GridError::InvalidLatitude)
    );
    assert_eq!(
        wgs84_to_unified(0.0, 181.0),
        Err(GridError::InvalidLongitude)
    );
    assert_eq!(
        wgs84_to_unified(0.0, -181.0),
        Err(GridError::InvalidLongitude)
    );
}
