use sightline_cot::cam_project::ray_to_pixel;
use sightline_cot::camera::Intrinsics;

#[test]
fn test_pinhole_roundtrip_grid() {
    let pinhole = Intrinsics::Pinhole {
        fx: 800.0,
        fy: 750.0,
        cx: 320.0,
        cy: 240.0,
    };
    for u_i in (0..=640).step_by(80) {
        for v_i in (0..=480).step_by(60) {
            let u = u_i as f64;
            let v = v_i as f64;
            let ray = pinhole.pixel_to_ray(u, v).unwrap();
            let (pu, pv) = ray_to_pixel(&pinhole, ray).unwrap();
            assert!((pu - u).abs() < 1e-6);
            assert!((pv - v).abs() < 1e-6);
        }
    }
}

#[test]
fn test_fisheye_roundtrip_grid() {
    let fisheye = Intrinsics::EquidistantFisheye {
        fx: 500.0,
        fy: 500.0,
        cx: 320.0,
        cy: 240.0,
        k1: -0.01,
        k2: 0.001,
    };
    for u_i in (40..=600).step_by(80) {
        for v_i in (40..=440).step_by(60) {
            let u = u_i as f64;
            let v = v_i as f64;
            let ray = fisheye.pixel_to_ray(u, v).unwrap();
            let (pu, pv) = ray_to_pixel(&fisheye, ray).unwrap();
            assert!((pu - u).abs() < 1e-6);
            assert!((pv - v).abs() < 1e-6);
        }
    }
}
