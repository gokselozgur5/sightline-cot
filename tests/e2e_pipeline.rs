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

fn convert_track_to_cot(
    trk: &Track,
    ref_lat: f64,
    ref_lon: f64,
    ref_alt: f64,
    time_ms: i64,
) -> CotEvent {
    let (lat_deg, lon_deg, hae_m) =
        enu_to_geodetic([trk.x[0], trk.x[1], trk.hae_m], ref_lat, ref_lon, ref_alt);
    let speed_mps = (trk.x[2].powi(2) + trk.x[3].powi(2)).sqrt();
    let course_deg = trk.x[2].atan2(trk.x[3]).to_degrees().rem_euclid(360.0);

    CotEvent {
        uid: format!("TRACK-{}", trk.id),
        cot_type: "a-f-G-E-V".into(),
        how: "m-g".into(),
        time_ms,
        stale_ms: time_ms + 10_000,
        lat_deg,
        lon_deg,
        hae_m,
        ce_m: (trk.p[0][0].sqrt() * 2.146).max(1.0),
        le_m: 5.0,
        course_deg,
        speed_mps,
    }
}

fn dispatch_tracks(dispatcher: &CotUdpDispatcher, tracks: &[Track], t_ms: i64) -> usize {
    let mut count = 0;
    for trk in tracks {
        let cot = convert_track_to_cot(trk, 37.7749, -122.4194, 0.0, t_ms);
        dispatcher.send_event(&cot).expect("dispatch");
        count += 1;
    }
    count
}
