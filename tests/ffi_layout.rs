use sightline_cot::ffi::*;
use std::mem::{align_of, size_of};

#[test]
fn camera_cfg_layout_is_stable() {
    assert_eq!(size_of::<SightlineCameraCfg>(), 152);
    assert_eq!(align_of::<SightlineCameraCfg>(), 8);
}

#[test]
fn detection_layout_is_stable() {
    assert_eq!(size_of::<SightlineDetectionC>(), 80);
    assert_eq!(align_of::<SightlineDetectionC>(), 8);
}

#[test]
fn track_layout_is_stable() {
    assert_eq!(size_of::<SightlineTrackC>(), 72);
    assert_eq!(align_of::<SightlineTrackC>(), 8);
}

#[test]
fn field_offsets_match_the_c_header() {
    let c = std::mem::MaybeUninit::<SightlineCameraCfg>::uninit();
    let base = c.as_ptr();
    let off = |p: *const u8| p as usize - base as usize;
    unsafe {
        assert_eq!(off(std::ptr::addr_of!((*base).is_fisheye) as *const u8), 48);
        assert_eq!(off(std::ptr::addr_of!((*base).min_hits) as *const u8), 136);
        assert_eq!(off(std::ptr::addr_of!((*base).q_acc) as *const u8), 144);
    }
    let t = std::mem::MaybeUninit::<SightlineTrackC>::uninit();
    let tb = t.as_ptr();
    let st = unsafe { std::ptr::addr_of!((*tb).status) } as usize - tb as usize;
    assert_eq!(st, 64);
}

#[test]
fn error_codes_are_distinct_and_negative() {
    let codes = [
        SIGHTLINE_ERR_NULL,
        SIGHTLINE_ERR_INVALID,
        SIGHTLINE_ERR_BUFFER,
        SIGHTLINE_ERR_PANIC,
        SIGHTLINE_ERR_CAMERA,
        SIGHTLINE_ERR_TRACKER,
    ];
    assert_eq!(SIGHTLINE_OK, 0);
    for (i, a) in codes.iter().enumerate() {
        assert!(*a < 0);
        assert!(codes[i + 1..].iter().all(|b| b != a));
    }
}
