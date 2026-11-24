use sightline_cot::ffi::*;

fn base_cfg() -> SightlineCameraCfg {
    SightlineCameraCfg {
        fx: 1000.0,
        fy: 1000.0,
        cx: 960.0,
        cy: 540.0,
        k1: 0.0,
        k2: 0.0,
        is_fisheye: 0,
        roll_rad: 0.0,
        pitch_rad: 0.0,
        yaw_rad: 0.0,
        cam_up_m: 600.0,
        ground_up_m: 0.0,
        pixel_sigma_px: 1.0,
        attitude_sigma_rad: 0.001,
        ref_lat_deg: 37.7749,
        ref_lon_deg: -122.4194,
        ref_hae_m: 100.0,
        min_hits: 1,
        max_age: 5,
        q_acc: 1.0,
    }
}

fn sample_det() -> SightlineDetectionC {
    SightlineDetectionC {
        u: 960.0,
        v: 540.0,
        x1: 950.0,
        y1: 530.0,
        x2: 970.0,
        y2: 550.0,
        embedding: [1.0f32, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    }
}

#[test]
fn test_engine_step_pose_and_dem() {
    let cfg = base_cfg();
    unsafe {
        let eng = sightline_engine_new(&cfg);
        assert!(!eng.is_null());
        let mut tracks = [SightlineTrackC::default(); 4];
        let mut out_n = 0usize;
        let det = sample_det();
        let pose = SightlinePoseC {
            roll_rad: 0.05,
            pitch_rad: -0.1,
            yaw_rad: 1.57,
            cam_up_m: 800.0,
            ref_lat_deg: 37.7800,
            ref_lon_deg: -122.4200,
            ref_hae_m: 120.0,
        };
        let rc = sightline_engine_step_pose(
            eng,
            0.033,
            &pose,
            &det,
            1,
            tracks.as_mut_ptr(),
            4,
            &mut out_n,
        );
        assert_eq!(rc, SIGHTLINE_OK);
        assert_eq!(out_n, 1);
        assert!(tracks[0].le90_m > 0.0 && tracks[0].le90_m != SIGHTLINE_UNKNOWN_ERROR_M);

        let elevs = [50.0; 9];
        let rc_dem = sightline_engine_set_dem(eng, -100.0, -100.0, 100.0, 3, 3, elevs.as_ptr());
        assert_eq!(rc_dem, SIGHTLINE_OK);

        let rc2 = sightline_engine_step_pose(
            eng,
            0.033,
            &pose,
            &det,
            1,
            tracks.as_mut_ptr(),
            4,
            &mut out_n,
        );
        assert_eq!(rc2, SIGHTLINE_OK);
        assert_eq!(sightline_engine_clear_dem(eng), SIGHTLINE_OK);
        sightline_engine_free(eng);
    }
}
