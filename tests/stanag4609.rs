use sightline_cot::stanag4609::StanagTimestamp;

#[test]
fn test_stanag_timestamp_micros_conversion() {
    let micros = 1_700_000_000_123_456_u64;
    let ts = StanagTimestamp::from_micros(micros);
    assert_eq!(ts.as_micros(), micros);
    assert_eq!(ts.to_cot_millis(), 1_700_000_000_123_i64);
}


#[test]
fn test_millis_and_secs_conversions() {
    let millis = 1_700_000_000_500_i64;
    let ts = StanagTimestamp::from_millis(millis).expect("valid positive millis");
    assert_eq!(ts.to_cot_millis(), millis);
    assert_eq!(ts.as_micros(), 1_700_000_000_500_000_u64);
    assert!((ts.to_secs_f64() - 1_700_000_000.5).abs() < 1e-6);

    assert!(StanagTimestamp::from_millis(-10).is_err());
}


#[test]
fn test_klv_tag2_roundtrip_encoding_and_decoding() {
    let ts = StanagTimestamp::from_micros(0x0102030405060708_u64);
    let tag2 = ts.to_klv_tag2();
    assert_eq!(tag2[0], 0x02);
    assert_eq!(tag2[1], 0x08);
    assert_eq!(&tag2[2..], &[1, 2, 3, 4, 5, 6, 7, 8]);

    let decoded = StanagTimestamp::from_klv_slice(&tag2).expect("valid KLV slice");
    assert_eq!(decoded, ts);
}


#[test]
fn test_iso8601_formatting_and_bidirectional_parsing() {
    let iso_str = "2026-11-16T08:41:00.123456Z";
    let ts = StanagTimestamp::from_iso8601(iso_str).expect("valid ISO string");
    let formatted = ts.to_iso8601_micros();
    assert_eq!(formatted, iso_str);
    assert_eq!(ts.to_iso8601_utc(), "2026-11-16T08:41:00.123Z");
}


#[test]
fn test_klv_error_handling() {
    let short_buf = [0x02, 0x08, 0x01];
    assert_eq!(StanagTimestamp::from_klv_slice(&short_buf), Err(sightline_cot::stanag4609::StanagError::BufferTooShort));

    let wrong_tag = [0x03, 0x08, 1, 2, 3, 4, 5, 6, 7, 8];
    assert!(matches!(StanagTimestamp::from_klv_slice(&wrong_tag), Err(sightline_cot::stanag4609::StanagError::InvalidKlvTag(0x03))));

    let wrong_len = [0x02, 0x04, 1, 2, 3, 4, 5, 6, 7, 8];
    assert!(matches!(StanagTimestamp::from_klv_slice(&wrong_len), Err(sightline_cot::stanag4609::StanagError::InvalidKlvLength(4))));
}
