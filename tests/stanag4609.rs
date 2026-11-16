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
