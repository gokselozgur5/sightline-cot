//! Tests for MGRS precision truncation.
use sightline_cot::mgrs_truncate::{truncate_mgrs, MgrsPrecision};

#[test]
fn test_truncate_mgrs_success() {
    let s = "31NEA1234567890";
    let p = MgrsPrecision::OneMeter;
    assert_eq!(truncate_mgrs(s, p).unwrap(), "31NEA1234567890");
    let p = MgrsPrecision::TenMeters;
    assert_eq!(truncate_mgrs(s, p).unwrap(), "31NEA12346789");
    let p = MgrsPrecision::HundredMeters;
    assert_eq!(truncate_mgrs(s, p).unwrap(), "31NEA123678");
    let p = MgrsPrecision::OneKilometer;
    assert_eq!(truncate_mgrs(s, p).unwrap(), "31NEA1267");
    let p = MgrsPrecision::TenKilometers;
    assert_eq!(truncate_mgrs(s, p).unwrap(), "31NEA16");
}
