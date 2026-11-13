//! Integration tests for 10-digit MGRS grid reference encoding.

use sightline_cot::mgrs_encode::encode_mgrs_10digit;

#[test]
fn test_encode_mgrs_10digit_equator() {
    let mgrs = encode_mgrs_10digit(0.0, 3.0).expect("valid coordinates");
    assert_eq!(mgrs.zone, 31);
    assert_eq!(mgrs.band, 'N');
    assert_eq!(mgrs.col, 'E');
    assert_eq!(mgrs.row, 'A');
    assert_eq!(mgrs.easting, 0);
    assert_eq!(mgrs.northing, 0);
    assert_eq!(mgrs.format_compact(), "31NEA0000000000");
    assert_eq!(mgrs.format_spaced(), "31N EA 00000 00000");
    assert_eq!(mgrs.to_string(), "31NEA0000000000");
}
