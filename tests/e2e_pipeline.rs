//! Deterministic End-to-End Multi-Target Pixel-to-CoT Integration Suite.

use sightline_cot::camera::{enu_to_geodetic, estimate_covariance_ce_le, intersect_ground};
use sightline_cot::camera::{CameraPose, Intrinsics};
use sightline_cot::cot::{CotEvent, CotUdpDispatcher};
use sightline_cot::tracker::{BoundingBox, Detection, MultiTargetTracker, Track, TrackStatus};
use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::Duration;

fn setup_camera() -> (Intrinsics, CameraPose) {
    let intrinsics = Intrinsics::Pinhole {
        fx: 1000.0,
        fy: 1000.0,
        cx: 960.0,
        cy: 540.0,
    };

    let pose = CameraPose {
        roll_rad: 0.0,
        pitch_rad: 0.0,
        yaw_rad: 0.0,
        cam_up_m: 1000.0,
    };

    (intrinsics, pose)
}

fn make_bbox(px: f64, py: f64) -> BoundingBox {
    BoundingBox {
        x1: px - 10.0,
        y1: py - 10.0,
        x2: px + 10.0,
        y2: py + 10.0,
    }
}

fn make_detection(
    px: f64,
    py: f64,
    intr: &Intrinsics,
    pose: &CameraPose,
    emb_idx: usize,
) -> Detection {
    let ray = intr.pixel_to_ray(px, py).expect("ray");
    let enu = intersect_ground(ray, pose, 0.0).expect("ground intersection");
    let (ce90_m, le90_m) =
        estimate_covariance_ce_le(intr, pose, px, py, 0.0, 1.0, 0.001).expect("cov");
    let mut embedding = [0.0f32; 8];
    embedding[emb_idx] = 1.0;

    Detection {
        east_m: enu[0],
        north_m: enu[1],
        hae_m: 10.0,
        ce90_m,
        le90_m,
        bbox: make_bbox(px, py),
        embedding,
    }
}
