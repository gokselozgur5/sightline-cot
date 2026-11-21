use sightline_cot::geohash::decode;
use sightline_cot::utm_string::parse_utm_string;

#[test]
fn geohash_rejects_non_ascii_letters() {
    // U+0131 (dotless i) truncates to 0x31 ('1') when cast to u8.
    assert!(decode("\u{131}").is_err());
}

#[test]
fn utm_string_rejects_non_ascii_band() {
    // U+0154 truncates to 0x54 ('T') when cast to u8.
    assert!(parse_utm_string("33\u{154} 500000 4649776").is_err());
}
