use sightline_cot::cot::CotEvent;
use sightline_cot::ffi::*;
use std::os::raw::c_char;

#[test]
fn exported_event_round_trips_with_stale_after_time() {
    let t = SightlineTrackC::default();
    let time_ms = 1_762_900_000_000;
    let mut buf = vec![0 as c_char; 4096];
    let rc = unsafe { sightline_track_to_cot_xml(&t, time_ms, buf.as_mut_ptr(), buf.len()) };
    assert_eq!(rc, SIGHTLINE_OK);
    let xml = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) }
        .to_str()
        .unwrap();
    let ev = CotEvent::from_xml(xml).expect("own output must parse");
    assert_eq!(ev.stale_ms, time_ms + 30_000);
}
