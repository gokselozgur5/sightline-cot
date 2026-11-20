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
    let frustum = res.unwrap();
    // At nadir (roll=0, pitch=0, yaw=0), +X_cam maps to East (+X_enu) and +Y_cam maps to South (-Y_enu).
    // Near plane corner 0 corresponds to pixel (0, 0): top-left -> West and North.
    assert!(frustum.corners_enu[0][0] < 0.0);
    assert!(frustum.corners_enu[0][1] > 0.0);
    // Near plane corner 2 corresponds to pixel (w, h): bottom-right -> East and South.
    assert!(frustum.corners_enu[2][0] > 0.0);
    assert!(frustum.corners_enu[2][1] < 0.0);
}

#[test]
fn test_camera_frustum_depth_plane_bbox() {
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
    let f = CameraFrustum::compute(&cam, &pose, 1920.0, 1080.0, 1.0, 100.0, 0.0, 0.0, 0.0).unwrap();
    assert!(f.enu_bbox.contains([0.0, 0.0, 0.0]));
}
