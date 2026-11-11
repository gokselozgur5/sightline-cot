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
