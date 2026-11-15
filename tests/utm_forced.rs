use sightline_cot::utm_forced::{utm_forced_to_wgs84, wgs84_to_utm_forced};

#[test]
fn test_utm_forced_across_adjacent_zones() {
    let lat = 51.5074;
    let lon = 0.1278;
    let forced = wgs84_to_utm_forced(lat, lon, 30).expect("force zone 30");
    assert_eq!(forced.zone, 30);
    assert_eq!(forced.hemisphere, 'N');
    let (lat_out, lon_out) = utm_forced_to_wgs84(&forced).expect("unproject");
    assert!((lat - lat_out).abs() < 1e-5);
    assert!((lon - lon_out).abs() < 1e-5);
}

#[test]
fn test_utm_forced_matches_standard_utm_in_native_zone() {
    let lat = 40.7128;
    let lon = -74.0060;
    let std_utm = sightline_cot::wgs84_to_utm(lat, lon).unwrap();
    let forced = wgs84_to_utm_forced(lat, lon, std_utm.zone).unwrap();
    assert_eq!(forced.zone, std_utm.zone);
    assert_eq!(forced.hemisphere, std_utm.hemisphere);
    assert!((forced.easting - std_utm.easting).abs() < 1e-4);
    assert!((forced.northing - std_utm.northing).abs() < 1e-4);
}
