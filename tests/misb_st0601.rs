use sightline_cot::misb_st0601::*;

#[test]
fn test_repro_overflow_and_ber_guards() {
    // Crafted BER length with huge byte count that would wrap
    let crafted_len = [0x88, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    assert_eq!(decode_ber_length(&crafted_len), Err(MisbError::InvalidBerLength));

    // Leading 0x80 in BER-OID tag is non-minimal
    let non_minimal_oid = [0x80, 0x01];
    assert_eq!(decode_ber_oid_tag(&non_minimal_oid), Err(MisbError::InvalidBerOidTag));

    // Shift overflow in BER-OID tag
    let huge_oid = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F];
    assert_eq!(decode_ber_oid_tag(&huge_oid), Err(MisbError::InvalidBerOidTag));
}

#[test]
fn test_repro_hfov_vfov_180_deg_scaling() {
    // Tag 16 (HFOV) with 0xFFFF should decode to 180.0 deg per MISB ST 0601, NOT 360.0 deg
    let hfov_max = [0x10, 0x02, 0xFF, 0xFF];
    let frame = Misb0601Frame::decode_local_set(&hfov_max).expect("valid hfov decode");
    assert!(
        (frame.sensor_hfov_deg.unwrap() - 180.0).abs() < 0.01,
        "Tag 16 should scale to 180.0 deg, got {}",
        frame.sensor_hfov_deg.unwrap()
    );

    // Tag 17 (VFOV) with 0xFFFF should decode to 180.0 deg per MISB ST 0601, NOT 360.0 deg
    let vfov_max = [0x11, 0x02, 0xFF, 0xFF];
    let frame_v = Misb0601Frame::decode_local_set(&vfov_max).expect("valid vfov decode");
    assert!(
        (frame_v.sensor_vfov_deg.unwrap() - 180.0).abs() < 0.01,
        "Tag 17 should scale to 180.0 deg, got {}",
        frame_v.sensor_vfov_deg.unwrap()
    );
}

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
    assert_eq!(decode_ber_length(&[0x85]), Err(MisbError::BufferTooShort));
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

#[test]
fn test_error_cases_and_sentinels() {
    let frame = Misb0601Frame {
        timestamp_micros: Some(1_000_000),
        ..Default::default()
    };
    let packet = encode_st0601_packet(&frame);

    let mut bad_key = packet.clone();
    bad_key[0] = 0xFF;
    assert_eq!(
        parse_st0601_packet(&bad_key),
        Err(MisbError::InvalidUniversalKey)
    );

    let mut bad_ck = packet.clone();
    let last = bad_ck.len() - 1;
    bad_ck[last] ^= 0xFF;
    assert_eq!(
        parse_st0601_packet(&bad_ck),
        Err(MisbError::InvalidChecksum)
    );

    assert_eq!(
        parse_st0601_packet(&packet[..20]),
        Err(MisbError::BufferTooShort)
    );

    let sentinel_pitch_tlv = [0x06, 0x02, 0x80, 0x00];
    assert_eq!(
        Misb0601Frame::decode_local_set(&sentinel_pitch_tlv),
        Err(MisbError::OutOfRange("sentinel or out of range i16"))
    );

    let sentinel_lat_tlv = [0x0D, 0x04, 0x80, 0x00, 0x00, 0x00];
    assert_eq!(
        Misb0601Frame::decode_local_set(&sentinel_lat_tlv),
        Err(MisbError::OutOfRange("sentinel or out of range i32"))
    );

    // Missing Tag 1 (0x01, 0x02) at packet end before checksum
    let mut missing_tag1 = packet.clone();
    let n = missing_tag1.len();
    missing_tag1[n - 4] = 0x02; // replace Tag 1 with Tag 2
    // recompute checksum so checksum check itself passes
    let new_ck = sightline_cot::stanag4609::compute_st0601_checksum(&missing_tag1[..n - 2]);
    missing_tag1[n - 2..].copy_from_slice(&new_ck.to_be_bytes());
    assert_eq!(
        parse_st0601_packet(&missing_tag1),
        Err(MisbError::MissingTag(1))
    );
}

#[test]
fn test_to_camera_pose_nadir_and_rays() {
    let mut frame = Misb0601Frame {
        sensor_true_alt_m: Some(1500.0),
        platform_heading_deg: Some(0.0),
        sensor_rel_az_deg: Some(0.0),
        sensor_rel_el_deg: Some(-90.0),
        ..Default::default()
    };

    let pose = frame.to_camera_pose(500.0).unwrap();
    assert_eq!(pose.cam_up_m, 1000.0);
    let ray_nadir = pose.transform_ray([0.0, 0.0, 1.0]);
    assert!((ray_nadir[0] - 0.0).abs() < 1e-9);
    assert!((ray_nadir[1] - 0.0).abs() < 1e-9);
    assert!((ray_nadir[2] - (-1.0)).abs() < 1e-9);

    frame.sensor_rel_el_deg = Some(-45.0);
    let pose_north = frame.to_camera_pose(500.0).unwrap();
    let ray_north = pose_north.transform_ray([0.0, 0.0, 1.0]);
    assert!((ray_north[0] - 0.0).abs() < 1e-9);
    assert!(ray_north[1] > 0.707);
    assert!(ray_north[2] < -0.707);

    frame.platform_heading_deg = Some(90.0);
    let pose_east = frame.to_camera_pose(500.0).unwrap();
    let ray_east = pose_east.transform_ray([0.0, 0.0, 1.0]);
    assert!(ray_east[0] > 0.707);
    assert!((ray_east[1] - 0.0).abs() < 1e-9);
    assert!(ray_east[2] < -0.707);

    frame.platform_heading_deg = Some(180.0);
    let pose_south = frame.to_camera_pose(500.0).unwrap();
    let ray_south = pose_south.transform_ray([0.0, 0.0, 1.0]);
    assert!((ray_south[0] - 0.0).abs() < 1e-9);
    assert!(ray_south[1] < -0.707);
    assert!(ray_south[2] < -0.707);

    frame.platform_heading_deg = Some(270.0);
    let pose_west = frame.to_camera_pose(500.0).unwrap();
    let ray_west = pose_west.transform_ray([0.0, 0.0, 1.0]);
    assert!(ray_west[0] < -0.707);
    assert!((ray_west[1] - 0.0).abs() < 1e-9);
    assert!(ray_west[2] < -0.707);

    frame.platform_heading_deg = Some(90.0);
    frame.sensor_rel_az_deg = Some(90.0);
    let pose_slew = frame.to_camera_pose(500.0).unwrap();
    let ray_slew = pose_slew.transform_ray([0.0, 0.0, 1.0]);
    assert!((ray_slew[0] - 0.0).abs() < 1e-9);
    assert!(ray_slew[1] < -0.707);
    assert!(ray_slew[2] < -0.707);
}
