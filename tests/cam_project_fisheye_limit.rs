use sightline_cot::cam_project::ray_to_pixel;
use sightline_cot::camera::Intrinsics;

#[test]
fn fisheye_projection_never_aliases_rays_past_the_monotonic_limit() {
    let cam = Intrinsics::EquidistantFisheye {
        fx: 100.0,
        fy: 100.0,
        cx: 0.0,
        cy: 0.0,
        k1: -0.3,
        k2: 0.0,
    };
    // r_d = theta - 0.3 theta^3 is monotonic only up to sqrt(1 / 0.9) = 1.054 rad.
    for theta in [0.5_f64, 1.0, 1.4, 1.8] {
        let ray = [theta.sin(), 0.0, theta.cos()];
        if let Ok((u, v)) = ray_to_pixel(&cam, ray) {
            let back = cam.pixel_to_ray(u, v).expect("pixel must unproject");
            assert!(
                (back[0] - ray[0]).abs() < 1e-6,
                "theta {theta}: u {u} -> {back:?}"
            );
        }
    }
    let behind = ray_to_pixel(&cam, [0.0, 0.0, -1.0]);
    assert!(!matches!(behind, Ok((u, _)) if u.is_nan()), "{behind:?}");
}
