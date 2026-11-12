use sightline_cot::ffi::*;
use std::os::raw::c_char;

fn xml_len(t: &SightlineTrackC) -> usize {
    let mut buf = vec![0 as c_char; 4096];
    let rc = unsafe { sightline_track_to_cot_xml(t, 1, buf.as_mut_ptr(), buf.len()) };
    assert_eq!(rc, SIGHTLINE_OK);
    unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) }
        .to_bytes()
        .len()
}

#[test]
fn zero_capacity_is_a_buffer_error() {
    let t = SightlineTrackC::default();
    let mut buf = [0x7f as c_char; 4];
    let rc = unsafe { sightline_track_to_cot_xml(&t, 1, buf.as_mut_ptr(), 0) };
    assert_eq!(rc, SIGHTLINE_ERR_BUFFER);
    assert!(buf.iter().all(|b| *b == 0x7f));
}
