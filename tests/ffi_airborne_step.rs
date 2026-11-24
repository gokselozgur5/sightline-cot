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

        let rc_dem_zero = sightline_engine_set_dem(eng, 0.0, 0.0, 10.0, 0, 0, std::ptr::null());
        assert_eq!(rc_dem_zero, SIGHTLINE_ERR_INVALID);

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

#[test]
fn test_airborne_moving_origin_track_consistency() {
    let mut cfg = base_cfg();
    cfg.ref_lat_deg = 37.7700;
    cfg.ref_lon_deg = -122.4200;
    cfg.ref_hae_m = 100.0;
    cfg.cam_up_m = 500.0;
    cfg.ground_up_m = 0.0;

    unsafe {
        let eng = sightline_engine_new(&cfg);
        assert!(!eng.is_null());
        let mut tracks = [SightlineTrackC::default(); 4];
        let mut out_n = 0usize;
        let det = sample_det();

        // Step 1: Nadir detection at ref origin
        let pose1 = SightlinePoseC {
            roll_rad: 0.0,
            pitch_rad: 0.0,
            yaw_rad: 0.0,
            cam_up_m: 500.0,
            ref_lat_deg: 37.7700,
            ref_lon_deg: -122.4200,
            ref_hae_m: 100.0,
        };
        let rc1 = sightline_engine_step_pose(
            eng,
            0.033,
            &pose1,
            &det,
            1,
            tracks.as_mut_ptr(),
            4,
            &mut out_n,
        );
        assert_eq!(rc1, SIGHTLINE_OK);
        assert_eq!(out_n, 1);
        let id1 = tracks[0].id;
        assert!((tracks[0].lat_deg - 37.7700).abs() < 1e-4);
        assert!((tracks[0].lon_deg - (-122.4200)).abs() < 1e-4);
        assert!((tracks[0].hae_m - 100.0).abs() < 0.5);

        // Step 2: Aircraft flies ~100m North (approx +0.0009 deg) and +20m up
        // With dets = NULL, n = 0 (coasting): Track must stay at (37.7700, -122.4200, 100.0) < 0.5m
        let pose2 = SightlinePoseC {
            roll_rad: 0.0,
            pitch_rad: 0.0,
            yaw_rad: 0.0,
            cam_up_m: 500.0,
            ref_lat_deg: 37.7709,
            ref_lon_deg: -122.4200,
            ref_hae_m: 120.0,
        };
        let rc2 = sightline_engine_step_pose(
            eng,
            0.033,
            &pose2,
            std::ptr::null(),
            0,
            tracks.as_mut_ptr(),
            4,
            &mut out_n,
        );
        assert_eq!(rc2, SIGHTLINE_OK);
        assert_eq!(out_n, 1);
        assert_eq!(tracks[0].id, id1);
        assert!(
            (tracks[0].lat_deg - 37.7700).abs() < 1e-5,
            "Track teleported in latitude: {}",
            tracks[0].lat_deg
        );
        assert!(
            (tracks[0].hae_m - 100.0).abs() < 0.5,
            "Track teleported in HAE: {}",
            tracks[0].hae_m
        );

        // Step 3: Failed step must NOT mutate pose or camera offset
        let rc_fail = sightline_engine_step_pose(
            eng,
            -1.0, // invalid dt_s
            &pose2,
            std::ptr::null(),
            0,
            tracks.as_mut_ptr(),
            4,
            &mut out_n,
        );
        assert_eq!(rc_fail, SIGHTLINE_ERR_INVALID);

        sightline_engine_free(eng);
    }
}

#[test]
fn test_airborne_stationary_ground_target_low_velocity() {
    let mut cfg = base_cfg();
    cfg.ref_lat_deg = 37.7700;
    cfg.ref_lon_deg = -122.4200;
    cfg.ref_hae_m = 100.0;
    cfg.cam_up_m = 500.0;
    cfg.ground_up_m = 0.0;

    unsafe {
        let eng = sightline_engine_new(&cfg);
        assert!(!eng.is_null());
        let mut tracks = [SightlineTrackC::default(); 4];
        let mut out_n = 0usize;

        // Frame 1: Nadir det at origin
        let pose1 = SightlinePoseC {
            roll_rad: 0.0,
            pitch_rad: 0.0,
            yaw_rad: 0.0,
            cam_up_m: 500.0,
            ref_lat_deg: 37.7700,
            ref_lon_deg: -122.4200,
            ref_hae_m: 100.0,
        };
        let det1 = sample_det(); // nadir u=cx, v=cy -> (0,0) ground
        let rc1 = sightline_engine_step_pose(
            eng,
            0.1,
            &pose1,
            &det1,
            1,
            tracks.as_mut_ptr(),
            4,
            &mut out_n,
        );
        assert_eq!(rc1, SIGHTLINE_OK);
        assert_eq!(out_n, 1);
        let id1 = tracks[0].id;

        // Aircraft moves 10m North (dlat ~ 10m / 111132m ~ 9.0e-5 deg).
        // Since aircraft moved 10m North, target is at North=0m, so camera hit is North=-10m.
        // v - cy = 20 => v = 560.0.
        let dlat_10m = 10.0 / 111132.954;
        let pose2 = SightlinePoseC {
            roll_rad: 0.0,
            pitch_rad: 0.0,
            yaw_rad: 0.0,
            cam_up_m: 500.0,
            ref_lat_deg: 37.7700 + dlat_10m,
            ref_lon_deg: -122.4200,
            ref_hae_m: 100.0,
        };
        let mut det2 = sample_det();
        det2.v = 560.0;

        let rc2 = sightline_engine_step_pose(
            eng,
            0.1,
            &pose2,
            &det2,
            1,
            tracks.as_mut_ptr(),
            4,
            &mut out_n,
        );
        assert_eq!(rc2, SIGHTLINE_OK);
        assert_eq!(out_n, 1);
        assert_eq!(tracks[0].id, id1);
        assert!(
            tracks[0].speed_mps < 1.0,
            "False aircraft velocity injected into stationary target: {} m/s",
            tracks[0].speed_mps
        );

        sightline_engine_free(eng);
    }
}
