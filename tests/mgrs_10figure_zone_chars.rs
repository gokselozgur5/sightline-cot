use sightline_cot::parse_mgrs_10figure;

#[test]
fn letters_in_the_zone_field_are_rejected() {
    assert!(parse_mgrs_10figure("J0NEA1234567890").is_err());
    assert!(parse_mgrs_10figure("JJNEA1234567890").is_err());
    assert!(parse_mgrs_10figure("ANEA1234567890").is_err());
}

#[test]
fn non_ascii_zone_bytes_are_rejected() {
    assert!(parse_mgrs_10figure("\u{e9}NEA1234567890").is_err());
}
