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
