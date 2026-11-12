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
