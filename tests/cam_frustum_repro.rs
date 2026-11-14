use sightline_cot::cam_frustum::CameraFrustum;
use sightline_cot::camera::{CameraPose, Intrinsics};

#[test]
fn test_camera_frustum_compiles() {
    let cam = Intrinsics::Pinhole {
        fx: 1000.0,
        fy: 1000.0,
        cx: 960.0,
        cy: 540.0,
    };
    let pose = CameraPose {
        roll_rad: 0.0,
        pitch_rad: 0.0,
        yaw_rad: 0.0,
        cam_up_m: 100.0,
    };
    let res = CameraFrustum::compute(&cam, &pose, 1920.0, 1080.0, 1.0, 100.0, 0.0, 0.0, 0.0);
    assert!(res.is_ok());
}
