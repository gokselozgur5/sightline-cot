use sightline_cot::camera::{BrownConrady, CameraError};

fn lens(k: [f64; 5]) -> BrownConrady {
    let [k1, k2, k3, p1, p2] = k;
    BrownConrady { k1, k2, k3, p1, p2 }
}

#[test]
fn test_identity_and_known_distortion_values() {
    assert_eq!(lens([0.0; 5]).distort(0.3, -0.4), (0.3, -0.4));
    let radial = lens([0.1, 0.01, 0.001, 0.0, 0.0]);
    let (xd, yd) = radial.distort(0.3, 0.4);
    let scale = 1.0 + 0.1 * 0.25 + 0.01 * 0.0625 + 0.001 * 0.015625;
    assert!((xd - 0.3 * scale).abs() < 1e-15 && (yd - 0.4 * scale).abs() < 1e-15);
    let tang = lens([0.0, 0.0, 0.0, 0.01, -0.02]);
    let (xt, yt) = tang.distort(0.3, 0.4);
    assert!((xt - (0.3 + 2.0 * 0.01 * 0.12 - 0.02 * (0.25 + 0.18))).abs() < 1e-15);
    assert!((yt - (0.4 + 0.01 * (0.25 + 0.32) - 2.0 * 0.02 * 0.12)).abs() < 1e-15);
    assert_eq!(radial.distort(0.0, 0.0), (0.0, 0.0));
}

const LENS: BrownConrady = BrownConrady {
    k1: -0.28,
    k2: 0.07,
    k3: -0.005,
    p1: 0.0012,
    p2: -0.0008,
};

#[test]
fn test_jacobian_matches_central_differences() {
    let h = 1e-6;
    for (x, y) in [(0.0, 0.0), (0.2, -0.1), (-0.5, 0.35), (0.6, 0.45)] {
        let j = LENS.jacobian(x, y);
        let (xp, yp) = LENS.distort(x + h, y);
        let (xm, ym) = LENS.distort(x - h, y);
        let (xq, yq) = LENS.distort(x, y + h);
        let (xn, yn) = LENS.distort(x, y - h);
        assert!((j[0][0] - (xp - xm) / (2.0 * h)).abs() < 1e-8);
        assert!((j[1][0] - (yp - ym) / (2.0 * h)).abs() < 1e-8);
        assert!((j[0][1] - (xq - xn) / (2.0 * h)).abs() < 1e-8);
        assert!((j[1][1] - (yq - yn) / (2.0 * h)).abs() < 1e-8);
    }
}

#[test]
fn test_undistort_round_trip_principal_point_to_border() {
    assert_eq!(LENS.undistort(0.0, 0.0, 20), Ok((0.0, 0.0)));
    let tiny = LENS.undistort(1e-12, -1e-12, 20).unwrap();
    assert!((tiny.0 - 1e-12).abs() < 1e-24 && (tiny.1 + 1e-12).abs() < 1e-24);
    for i in -8..=8 {
        for j in -6..=6 {
            let (x, y) = (i as f64 * 0.09, j as f64 * 0.09);
            let (xd, yd) = LENS.distort(x, y);
            let (xu, yu) = LENS.undistort(xd, yd, 20).unwrap();
            assert!((xu - x).abs() < 1e-12 && (yu - y).abs() < 1e-12);
        }
    }
}

#[test]
fn test_undistort_failure_paths() {
    let invalid = Err(CameraError::InvalidParameters);
    assert_eq!(LENS.undistort(f64::NAN, 0.1, 20), invalid);
    assert_eq!(LENS.undistort(0.1, f64::INFINITY, 20), invalid);
    let bad = lens([-0.28, f64::NAN, 0.0, 0.0, 0.0]);
    assert_eq!(bad.undistort(0.1, 0.1, 20), invalid);
    let failed = Err(CameraError::DistortionSolverFailed);
    assert_eq!(LENS.undistort(0.3, 0.2, 0), failed);
    assert_eq!(LENS.undistort(0.3, 0.2, 1), failed);
    let barrel = lens([-0.5, 0.0, 0.0, 0.0, 0.0]);
    assert_eq!(barrel.undistort(0.6, 0.0, 50), failed);
    assert_eq!(barrel.undistort(0.0, 1e200, 50), failed);
}
