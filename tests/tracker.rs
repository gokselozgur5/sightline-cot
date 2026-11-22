use sightline_cot::cam_frustum::CameraFrustum;
use sightline_cot::camera::{CameraPose, Intrinsics};
use sightline_cot::track_assoc::AssocConfig;
use sightline_cot::tracker::MultiTargetTracker;

#[test]
fn test_repro_390_tracker_assoc_config_and_frustum_error() {
    let cfg = AssocConfig {
        gate_threshold: 50.0,
        spatial_weight: 1.0,
        iou_weight: 100.0,
        cosine_weight: 5.0,
    };
    let tracker = MultiTargetTracker::with_assoc_config(3, 5, 2.0, cfg);
    assert_eq!(tracker.assoc_config.iou_weight, 100.0);

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
    let _: Result<CameraFrustum, sightline_cot::sensor_frustum::FrustumError> = res;
}
