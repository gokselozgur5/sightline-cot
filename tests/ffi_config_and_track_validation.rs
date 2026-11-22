use sightline_cot::ffi::*;

#[test]
fn engine_new_rejects_a_camera_on_the_ground_plane() {
    let cfg = SightlineCameraCfg {
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
        cam_up_m: 0.0,
        ground_up_m: 0.0,
        pixel_sigma_px: 1.0,
        attitude_sigma_rad: 0.001,
        ref_lat_deg: 37.0,
        ref_lon_deg: -122.0,
        ref_hae_m: 0.0,
        min_hits: 1,
        max_age: 5,
        q_acc: 1.0,
    };
    let e = unsafe { sightline_engine_new(&cfg) };
    let accepted = !e.is_null();
    unsafe { sightline_engine_free(e) };
    assert!(
        !accepted,
        "cam_up_m == ground_up_m can never locate a detection"
    );
}

#[test]
fn track_to_cot_xml_rejects_non_finite_coordinates() {
    let t = SightlineTrackC {
        lat_deg: f64::NAN,
        ..SightlineTrackC::default()
    };
    let mut buf = [0 as std::os::raw::c_char; 1024];
    let rc = unsafe { sightline_track_to_cot_xml(&t, 0, buf.as_mut_ptr(), buf.len()) };
    assert_eq!(rc, SIGHTLINE_ERR_INVALID);
}
