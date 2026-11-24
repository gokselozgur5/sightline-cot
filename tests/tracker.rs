use sightline_cot::cam_frustum::CameraFrustum;
use sightline_cot::camera::{CameraPose, Intrinsics};
use sightline_cot::track_assoc::AssocConfig;
use sightline_cot::tracker::{BoundingBox, Detection, MultiTargetTracker};

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

#[test]
fn test_repro_tracker_assoc_config_wiring() {
    let det_init = Detection {
        east_m: 0.0,
        north_m: 0.0,
        hae_m: 0.0,
        ce90_m: 1.5,
        le90_m: 1.0,
        bbox: BoundingBox {
            x1: 0.0,
            y1: 0.0,
            x2: 10.0,
            y2: 10.0,
        },
        embedding: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    };

    let det_match = Detection {
        east_m: 0.1,
        north_m: 0.1,
        hae_m: 0.0,
        ce90_m: 1.5,
        le90_m: 1.0,
        bbox: BoundingBox {
            x1: 0.0,
            y1: 0.0,
            x2: 10.0,
            y2: 10.0,
        },
        embedding: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    };

    // Tracker with very tight gate_threshold (e.g. 0.0001) should reject association.
    // In un-wired tracker, gate_threshold is ignored and association succeeds.
    let tight_cfg = AssocConfig {
        gate_threshold: 0.0001,
        spatial_weight: 1.0,
        iou_weight: 10.0,
        cosine_weight: 5.0,
    };
    let mut tracker_tight = MultiTargetTracker::with_assoc_config(1, 5, 0.5, tight_cfg);
    tracker_tight.step(1.0, &[det_init.clone()]).unwrap();
    tracker_tight.step(1.0, &[det_match.clone()]).unwrap();
    assert_eq!(
        tracker_tight.tracks.len(),
        2,
        "Tight gate_threshold must reject association"
    );
}
