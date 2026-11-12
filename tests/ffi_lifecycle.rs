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
