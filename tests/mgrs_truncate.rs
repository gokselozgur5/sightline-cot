//! Integration tests for MGRS precision truncation.

use sightline_cot::mgrs_truncate::{truncate_mgrs, MgrsPrecision};

#[test]
fn test_truncate_mgrs_metric_precisions() {
    let full = "31NEA0123456789";
    assert_eq!(
        truncate_mgrs(full, MgrsPrecision::Point1m).unwrap(),
        "31NEA0123456789"
    );
    assert_eq!(
        truncate_mgrs(full, MgrsPrecision::Point10m).unwrap(),
        "31NEA01235678"
    );
    assert_eq!(
        truncate_mgrs(full, MgrsPrecision::Point100m).unwrap(),
        "31NEA012567"
    );
}

#[test]
fn test_truncate_mgrs_kilometer_precisions() {
    let full = "31NEA0123456789";
    assert_eq!(
        truncate_mgrs(full, MgrsPrecision::Point1k).unwrap(),
        "31NEA0156"
    );
    assert_eq!(
        truncate_mgrs(full, MgrsPrecision::Point10k).unwrap(),
        "31NEA05"
    );
    assert_eq!(
        truncate_mgrs(full, MgrsPrecision::Point100k).unwrap(),
        "31NEA"
    );
}
