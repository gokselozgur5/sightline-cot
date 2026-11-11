use sightline_cot::camera::{
    enu_to_geodetic, intersect_ground, CameraError, CameraPose, Intrinsics,
};
use std::f64::consts::PI;

#[test]
fn test_wgs84_extreme_coordinates_accuracy() {
    let origins = [
        (0.0, 0.0, 0.0),
        (0.0, 179.9999, 100.0),
        (0.0, -179.9999, 100.0),
        (89.9999, 0.0, 500.0),
        (-89.9999, 0.0, 500.0),
        (31.5, 35.5, -430.0),
    ];
    for (lat0, lon0, hae0) in origins {
        let (lat_z, lon_z, hae_z) = enu_to_geodetic([0.0, 0.0, 0.0], lat0, lon0, hae0);
        assert!((lat_z - lat0).abs() < 1e-7);
        assert!((lon_z - lon0).abs() < 1e-7);
        assert!((hae_z - hae0).abs() < 1e-4);

        let (lat_up, lon_up, hae_up) = enu_to_geodetic([0.0, 0.0, 100.0], lat0, lon0, hae0);
        assert!((lat_up - lat0).abs() < 1e-6);
        assert!((lon_up - lon0).abs() < 1e-6);
        assert!((hae_up - (hae0 + 100.0)).abs() < 1e-3);
    }
}

#[test]
fn test_wgs84_antimeridian_wrap_and_equator_oracle() {
    let (_, lon_wrap, _) = enu_to_geodetic([1000.0, 0.0, 0.0], 0.0, 179.9999, 0.0);
    assert!((-180.0..=180.0).contains(&lon_wrap));
    assert!((lon_wrap - -179.99112).abs() < 1e-5);

    let (lat_n, lon_e, _) = enu_to_geodetic([1000.0, 1000.0, 0.0], 0.0, 0.0, 0.0);
    assert!((lat_n - (1000.0 / 110574.39)).abs() < 1e-6);
    assert!((lon_e - (1000.0 / 111319.49)).abs() < 1e-6);

    let (lat_ds, lon_ds, hae_ds) = enu_to_geodetic([500.0, -500.0, 0.0], 31.5, 35.5, -430.0);
    assert!(lat_ds.is_finite() && lon_ds.is_finite());
    assert!((hae_ds - -430.0).abs() < 0.1);
}

#[test]
fn test_polar_singularity_offsets() {
    for pole_lat in [90.0, -90.0] {
        let (lat, lon, hae) = enu_to_geodetic([0.0, 0.0, 50.0], pole_lat, 0.0, 0.0);
        assert!((lat - pole_lat).abs() < 1e-5);
        assert!(lon.is_finite());
        assert!((hae - 50.0).abs() < 1e-3);

        let (lat_off, lon_off, hae_off) = enu_to_geodetic([100.0, 100.0, 0.0], pole_lat, 0.0, 0.0);
        assert!(lat_off.is_finite() && lon_off.is_finite() && hae_off.is_finite());
        let lat_drop = (pole_lat.abs() - lat_off.abs()).abs();
        let expected_drop = 2.0_f64.sqrt() * 100.0 / 111133.0;
        assert!((lat_drop - expected_drop).abs() < 1e-5);
    }
}

fn angle_diff(a: f64, b: f64) -> f64 {
    let mut d = (a - b) % (2.0 * PI);
    if d > PI {
        d -= 2.0 * PI;
    } else if d < -PI {
        d += 2.0 * PI;
    }
    d
}

fn check_yaw_norm(cam: &Intrinsics, uv: (f64, f64), roll: f64, pitch: f64) {
    let ray = cam.pixel_to_ray(uv.0, uv.1).unwrap();
    let p0 = CameraPose {
        roll_rad: roll,
        pitch_rad: pitch,
        yaw_rad: 0.0,
        cam_up_m: 100.0,
    };
    let b_pt = intersect_ground(ray, &p0, 0.0).unwrap();
    let b_rng = (b_pt[0].powi(2) + b_pt[1].powi(2)).sqrt();
    let b_brg = b_pt[0].atan2(b_pt[1]);
    for s in 1..36 {
        let y = (s as f64) * (PI / 18.0);
        let p = CameraPose {
            roll_rad: roll,
            pitch_rad: pitch,
            yaw_rad: y,
            cam_up_m: 100.0,
        };
        let pt = intersect_ground(ray, &p, 0.0).unwrap();
        let rng = (pt[0].powi(2) + pt[1].powi(2)).sqrt();
        assert!((rng - b_rng).abs() < 1e-5);
        assert!(angle_diff(pt[0].atan2(pt[1]), b_brg - y).abs() < 1e-5);
    }
}

#[test]
fn test_so3_yaw_sweep_horizontal_norm_invariance() {
    let pinhole = Intrinsics::Pinhole {
        fx: 800.0,
        fy: 800.0,
        cx: 640.0,
        cy: 360.0,
    };
    check_yaw_norm(&pinhole, (640.0, 360.0), 0.0, 0.5);
    check_yaw_norm(&pinhole, (900.0, 200.0), 0.1, 0.6);
}

#[test]
fn test_so3_fisheye_yaw_sweep() {
    let fisheye = Intrinsics::EquidistantFisheye {
        fx: 600.0,
        fy: 600.0,
        cx: 640.0,
        cy: 360.0,
        k1: 0.01,
        k2: -0.001,
    };
    check_yaw_norm(&fisheye, (750.0, 450.0), -0.05, 0.55);
}

#[test]
fn test_grazing_angle_horizon_boundary() {
    let cam = Intrinsics::Pinhole {
        fx: 1000.0,
        fy: 1000.0,
        cx: 500.0,
        cy: 500.0,
    };
    let ray = cam.pixel_to_ray(500.0, 500.0).unwrap();
    let eps = 1e-4;
    let p_below = CameraPose {
        roll_rad: 0.0,
        pitch_rad: PI / 2.0 - eps,
        yaw_rad: 0.0,
        cam_up_m: 100.0,
    };
    let pt = intersect_ground(ray, &p_below, 0.0).unwrap();
    let expected = 100.0 / eps.tan();
    let computed = (pt[0].powi(2) + pt[1].powi(2)).sqrt();
    assert!(((computed - expected) / expected).abs() < 1e-4);
}
