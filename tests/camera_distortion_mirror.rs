use sightline_cot::camera::{BrownConrady, CameraError};

#[test]
fn test_undistort_rejects_mirrored_roots() {
    let lens = BrownConrady { k1: -0.5, k2: 0.0, k3: 0.0, p1: 0.0, p2: 0.0 };
    let failed = Err(CameraError::DistortionSolverFailed);
    assert_eq!(lens.undistort(-57.5, 0.0, 50), failed);
    assert_eq!(lens.undistort(-490.0, 0.0, 50), failed);
    let (x, _) = lens.undistort(-0.448, 0.0, 50).unwrap();
    assert!(x.abs() < 0.816);
}
