//! Tests for MGRS string parser to WGS84.

use sightline_cot::mgrs_parse::{parse_mgrs, parse_mgrs_to_wgs84};

#[test]
fn test_parse_mgrs_basic() {
    let s = "31NEA1234567890";
    let parsed = parse_mgrs(s).unwrap();
    assert_eq!(parsed.zone, 31);
    assert_eq!(parsed.band, 'N');
    assert_eq!(parsed.col, 'E');
    assert_eq!(parsed.row, 'A');
    assert_eq!(parsed.precision, 5);
}

#[test]
fn test_parse_mgrs_to_wgs84_repro() {
    let (lat, lon) = parse_mgrs_to_wgs84("31NEA0000000000").unwrap();
    assert!((lat - 0.0).abs() < 1e-4);
    assert!((lon - 3.0).abs() < 1e-4);
}

#[test]
fn test_parse_mgrs_spaced_and_case_insensitive() {
    let s = "  31n   ea   12345   67890  ";
    let p = parse_mgrs(s).unwrap();
    assert_eq!(p.zone, 31);
    assert_eq!(p.band, 'N');
    assert_eq!(p.col, 'E');
    assert_eq!(p.row, 'A');
    assert_eq!(p.easting, 12345);
    assert_eq!(p.northing, 67890);
    assert_eq!(p.precision, 5);
    assert_eq!(p.format_compact(), "31NEA1234567890");
    assert_eq!(p.format_spaced(), "31N EA 12345 67890");
}
