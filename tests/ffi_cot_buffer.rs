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

#[test]
fn capacity_one_short_of_nul_is_rejected_without_writing() {
    let t = SightlineTrackC::default();
    let len = xml_len(&t);
    let mut buf = vec![0x7f as c_char; len + 2];
    let rc = unsafe { sightline_track_to_cot_xml(&t, 1, buf.as_mut_ptr(), len) };
    assert_eq!(rc, SIGHTLINE_ERR_BUFFER);
    assert!(buf.iter().all(|b| *b == 0x7f));
}

#[test]
fn exact_capacity_writes_nul_and_stays_in_bounds() {
    let t = SightlineTrackC::default();
    let len = xml_len(&t);
    let mut buf = vec![0x7f as c_char; len + 2];
    let rc = unsafe { sightline_track_to_cot_xml(&t, 1, buf.as_mut_ptr(), len + 1) };
    assert_eq!(rc, SIGHTLINE_OK);
    assert_eq!(buf[len], 0);
    assert_eq!(buf[len + 1], 0x7f);
}
