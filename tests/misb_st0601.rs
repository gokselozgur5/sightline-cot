use sightline_cot::misb_st0601::*;

#[test]
fn test_ber_length_short_and_long() {
    let mut buf = Vec::new();
    encode_ber_length(10, &mut buf);
    assert_eq!(buf, vec![10]);
    let (len, consumed) = decode_ber_length(&buf).unwrap();
    assert_eq!((len, consumed), (10, 1));

    let mut buf2 = Vec::new();
    encode_ber_length(300, &mut buf2);
    assert_eq!(buf2, vec![0x82, 0x01, 0x2C]);
    let (len2, consumed2) = decode_ber_length(&buf2).unwrap();
    assert_eq!((len2, consumed2), (300, 3));
}

#[test]
fn test_ber_length_invalid() {
    assert_eq!(decode_ber_length(&[]), Err(MisbError::BufferTooShort));
    assert_eq!(decode_ber_length(&[0x80]), Err(MisbError::InvalidBerLength));
    assert_eq!(
        decode_ber_length(&[0x85]),
        Err(MisbError::BufferTooShort)
    );
}

#[test]
fn test_ber_oid_tag() {
    let mut buf = Vec::new();
    encode_ber_oid_tag(13, &mut buf);
    assert_eq!(buf, vec![13]);
    let (tag, consumed) = decode_ber_oid_tag(&buf).unwrap();
    assert_eq!((tag, consumed), (13, 1));

    let mut buf2 = Vec::new();
    encode_ber_oid_tag(300, &mut buf2);
    let (tag2, consumed2) = decode_ber_oid_tag(&buf2).unwrap();
    assert_eq!((tag2, consumed2), (300, buf2.len()));
}

#[test]
fn test_round_trip_all_tags() {
    let frame = Misb0601Frame {
        timestamp_micros: Some(1_700_000_000_000_000),
        platform_heading_deg: Some(123.45),
        platform_pitch_deg: Some(-12.34),
        platform_roll_deg: Some(25.67),
        sensor_lat_deg: Some(37.7749),
        sensor_lon_deg: Some(-122.4194),
        sensor_true_alt_m: Some(2500.5),
        sensor_hfov_deg: Some(30.0),
        sensor_vfov_deg: Some(20.0),
        sensor_rel_az_deg: Some(45.0),
        sensor_rel_el_deg: Some(-60.0),
        sensor_rel_roll_deg: Some(5.0),
        slant_range_m: Some(12345.0),
        target_width_m: Some(250.0),
        frame_center_lat_deg: Some(37.8000),
        frame_center_lon_deg: Some(-122.4000),
        frame_center_elev_m: Some(50.0),
    };

    let packet = encode_st0601_packet(&frame);
    let parsed = parse_st0601_packet(&packet).expect("valid parse");

    assert_eq!(parsed.timestamp_micros, frame.timestamp_micros);
    assert!((parsed.platform_heading_deg.unwrap() - 123.45).abs() < 0.01);
    assert!((parsed.platform_pitch_deg.unwrap() - (-12.34)).abs() < 0.01);
    assert!((parsed.platform_roll_deg.unwrap() - 25.67).abs() < 0.01);
    assert!((parsed.sensor_lat_deg.unwrap() - 37.7749).abs() < 1e-4);
    assert!((parsed.sensor_lon_deg.unwrap() - (-122.4194)).abs() < 1e-4);
    assert!((parsed.sensor_true_alt_m.unwrap() - 2500.5).abs() < 1.0);
    assert!((parsed.sensor_hfov_deg.unwrap() - 30.0).abs() < 0.01);
    assert!((parsed.sensor_vfov_deg.unwrap() - 20.0).abs() < 0.01);
    assert!((parsed.sensor_rel_az_deg.unwrap() - 45.0).abs() < 0.01);
    assert!((parsed.sensor_rel_el_deg.unwrap() - (-60.0)).abs() < 0.01);
    assert!((parsed.sensor_rel_roll_deg.unwrap() - 5.0).abs() < 0.01);
    assert!((parsed.slant_range_m.unwrap() - 12345.0).abs() < 2.0);
    assert!((parsed.target_width_m.unwrap() - 250.0).abs() < 1.0);
    assert!((parsed.frame_center_lat_deg.unwrap() - 37.8000).abs() < 1e-4);
    assert!((parsed.frame_center_lon_deg.unwrap() - (-122.4000)).abs() < 1e-4);
    assert!((parsed.frame_center_elev_m.unwrap() - 50.0).abs() < 1.0);
}

