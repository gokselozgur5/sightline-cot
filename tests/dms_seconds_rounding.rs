use sightline_cot::geo_dms::{format_dms_lat, parse_dms_lat};

#[test]
fn seconds_field_never_prints_sixty() {
    let s = format_dms_lat(0.9999999).unwrap();
    assert!(!s.contains("60.00"), "formatted as {s}");
    assert!(parse_dms_lat(&s).is_ok(), "own output rejected: {s}");
}
