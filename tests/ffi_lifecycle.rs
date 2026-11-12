use sightline_cot::ffi::*;

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
fn free_null_is_a_no_op_and_repeated_cycles_succeed() {
    let c = cfg();
    unsafe {
        sightline_engine_free(std::ptr::null_mut());
        for _ in 0..64 {
            let e = sightline_engine_new(&c);
            assert!(!e.is_null());
            sightline_engine_free(e);
        }
    }
}

#[test]
fn engines_are_independent_handles() {
    let c = cfg();
    let d = det();
    let (mut out, mut n) = ([SightlineTrackC::default(); 2], 0usize);
    unsafe {
        let a = sightline_engine_new(&c);
        let b = sightline_engine_new(&c);
        assert_ne!(a, b);
        let rc = sightline_engine_step(a, 0.1, &d, 1, out.as_mut_ptr(), 2, &mut n);
        assert_eq!((rc, n), (SIGHTLINE_OK, 1));
        sightline_engine_free(a);
        let rc = sightline_engine_step(b, 0.1, std::ptr::null(), 0, out.as_mut_ptr(), 2, &mut n);
        assert_eq!((rc, n), (SIGHTLINE_OK, 0));
        sightline_engine_free(b);
    }
}
