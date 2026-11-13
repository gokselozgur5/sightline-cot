//! Integration tests for UTM coordinate string formatting and parsing.

use sightline_cot::utm_string::{format_utm_string, parse_utm_string};

#[test]
fn test_utm_string_parse_and_format_roundtrip() {
    let s = "33T 500000 4649776";
    let coord = parse_utm_string(s).expect("valid utm string");
    assert_eq!(coord.zone, 33);
    assert_eq!(coord.band, 'T');
    assert_eq!(coord.easting, 500000.0);
    assert_eq!(coord.northing, 4649776.0);
    assert_eq!(format_utm_string(&coord), s);
    assert_eq!(coord.to_string(), s);
}

#[test]
fn test_utm_string_rejects_bad_zone_and_band() {
    assert!(parse_utm_string("0T 500000 4649776").is_err());
    assert!(parse_utm_string("61T 500000 4649776").is_err());
    assert!(parse_utm_string("33I 500000 4649776").is_err());
    assert!(parse_utm_string("33O 500000 4649776").is_err());
    assert!(parse_utm_string("33Z 500000 4649776").is_err());
    assert!(parse_utm_string("33AB 500000 4649776").is_err());
}
