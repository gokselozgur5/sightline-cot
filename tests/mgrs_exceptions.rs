//! Grid zone exceptions for southwest Norway and Svalbard.

use sightline_cot::{format_mgrs, utm_to_wgs84, wgs84_to_utm};

fn zone(lat: f64, lon: f64) -> u8 {
    wgs84_to_utm(lat, lon).unwrap().zone
}

#[test]
fn norway_32v_widened() {
    assert_eq!(zone(60.0, 5.0), 32);
    assert_eq!(zone(60.0, 3.0), 32);
    assert_eq!(zone(60.0, 2.9999), 31);
    assert_eq!(zone(60.0, 11.9999), 32);
    assert_eq!(zone(60.0, 12.0), 33);
    assert_eq!(zone(55.9999, 5.0), 31);
    assert_eq!(zone(56.0, 5.0), 32);
    assert_eq!(zone(63.9999, 5.0), 32);
    assert_eq!(zone(64.0, 5.0), 31);
    assert!(format_mgrs(60.0, 5.0, 5)
        .unwrap()
        .as_str()
        .starts_with("32V"));
}
