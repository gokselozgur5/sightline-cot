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
