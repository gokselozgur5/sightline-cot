use sightline_cot::ffi::*;
use std::os::raw::c_char;

#[test]
fn test_typed_cot_and_tak_proto_ffi() {
    let track = SightlineTrackC {
        id: 42,
        lat_deg: 37.7749,
        lon_deg: -122.4194,
        hae_m: 150.0,
        ce90_m: 2.5,
        le90_m: 3.5,
        course_deg: 90.0,
        speed_mps: 15.0,
        status: SIGHTLINE_STATUS_CONFIRMED,
    };

    let cot_type = sightline_cot_type_from_class(1, 0); // Hostile Vehicle -> a-h-G-E-V
    assert!(!cot_type.is_null());
    let type_str = unsafe { std::ffi::CStr::from_ptr(cot_type) }
        .to_str()
        .unwrap();
    assert_eq!(type_str, "a-h-G-E-V");

    let mut xml_buf = [0 as c_char; 2048];
    let rc_xml = unsafe {
        sightline_track_to_cot_xml_typed(
            &track,
            1_700_000_000_000,
            cot_type,
            xml_buf.as_mut_ptr(),
            xml_buf.len(),
        )
    };
    assert_eq!(rc_xml, SIGHTLINE_OK);
    let xml_str = unsafe { std::ffi::CStr::from_ptr(xml_buf.as_ptr()) }
        .to_str()
        .unwrap();
    assert!(xml_str.contains("type=\"a-h-G-E-V\""));

    let mut proto_buf = [0u8; 1024];
    let mut out_len = 0usize;
    let rc_proto = unsafe {
        sightline_track_to_tak_proto(
            &track,
            1_700_000_000_000,
            cot_type,
            proto_buf.as_mut_ptr(),
            proto_buf.len(),
            &mut out_len,
        )
    };
    assert_eq!(rc_proto, SIGHTLINE_OK);
    assert!(out_len > 0);
    assert_eq!(proto_buf[0], 0xBF);

    let mut tiny_buf = [0u8; 4];
    let mut req_len = 0usize;
    let rc_small = unsafe {
        sightline_track_to_tak_proto(
            &track,
            1_700_000_000_000,
            cot_type,
            tiny_buf.as_mut_ptr(),
            tiny_buf.len(),
            &mut req_len,
        )
    };
    assert_eq!(rc_small, SIGHTLINE_ERR_BUFFER);
    assert_eq!(req_len, out_len);
}
