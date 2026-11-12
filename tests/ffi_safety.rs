use sightline_cot::ffi::*;
use std::os::raw::c_char;

fn cfg() -> SightlineCameraCfg {
    SightlineCameraCfg {
        fx: 1000.0,
        fy: 1000.0,
        cx: 500.0,
        cy: 500.0,
        k1: 0.0,
        k2: 0.0,
        is_fisheye: 0,
        roll_rad: 0.0,
        pitch_rad: 0.0,
        yaw_rad: 0.0,
        cam_up_m: 100.0,
        ground_up_m: 0.0,
        pixel_sigma_px: 1.0,
        attitude_sigma_rad: 0.001,
        ref_lat_deg: 37.0,
        ref_lon_deg: -122.0,
        ref_hae_m: 10.0,
        min_hits: 1,
        max_age: 3,
        q_acc: 0.5,
    }
}

fn det() -> SightlineDetectionC {
    SightlineDetectionC {
        u: 520.0,
        v: 480.0,
        x1: 500.0,
        y1: 460.0,
        x2: 540.0,
        y2: 500.0,
        embedding: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    }
}

#[test]
fn step_with_zero_detections_and_zero_capacity_succeeds() {
    let c = cfg();
    let mut n = 99usize;
    unsafe {
        let e = sightline_engine_new(&c);
        assert!(!e.is_null());
        let rc =
            sightline_engine_step(e, 0.1, std::ptr::null(), 0, std::ptr::null_mut(), 0, &mut n);
        assert_eq!((rc, n), (SIGHTLINE_OK, 0));
        sightline_engine_free(e);
    }
}

#[test]
fn step_rejects_null_out_count_and_null_detections() {
    let c = cfg();
    let d = det();
    let mut n = 0usize;
    unsafe {
        let e = sightline_engine_new(&c);
        let out = std::ptr::null_mut();
        let rc = sightline_engine_step(e, 0.1, &d, 1, out, 0, std::ptr::null_mut());
        assert_eq!(rc, SIGHTLINE_ERR_NULL);
        let rc = sightline_engine_step(e, 0.1, std::ptr::null(), 1, out, 0, &mut n);
        assert_eq!(rc, SIGHTLINE_ERR_NULL);
        sightline_engine_free(e);
    }
}
