use sightline_cot::stanag4609::StanagTimestamp;

#[test]
fn test_stanag_timestamp_micros_conversion() {
    let micros = 1_700_000_000_123_456_u64;
    let ts = StanagTimestamp::from_micros(micros);
    assert_eq!(ts.as_micros(), micros);
    assert_eq!(ts.to_cot_millis(), 1_700_000_000_123_i64);
}
