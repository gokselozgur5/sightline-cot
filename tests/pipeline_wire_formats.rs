//! Tracker output carried through CoT XML and TAK protobuf framing.

use sightline_cot::cot::CotEvent;
use sightline_cot::proto::{decode_tak_packet, encode_tak_packet, TakCotEvent};
use sightline_cot::tracker::{BoundingBox, Detection, MultiTargetTracker, TrackStatus};

fn det(east_m: f64, north_m: f64, idx: usize) -> Detection {
    let mut embedding = [0.0f32; 8];
    embedding[idx] = 1.0;
    let bbox = BoundingBox {
        x1: east_m - 5.0,
        y1: north_m - 5.0,
        x2: east_m + 5.0,
        y2: north_m + 5.0,
    };
    Detection {
        east_m,
        north_m,
        hae_m: 10.0,
        ce90_m: 2.0,
        le90_m: 2.0,
        bbox,
        embedding,
    }
}
fn confirmed_events() -> Vec<CotEvent> {
    let mut tracker = MultiTargetTracker::new(2, 5, 1.0);
    for f in 0..4 {
        let d = f as f64;
        tracker
            .step(0.1, &[det(10.0 + d, 20.0, 0), det(300.0, 400.0 - d, 1)])
            .expect("step");
    }
    let t0 = 1_771_234_500_000;
    let mut out = Vec::new();
    for t in tracker
        .tracks
        .iter()
        .filter(|t| t.status == TrackStatus::Confirmed)
    {
        let (lat, lon) = (37.7749 + t.x[1] * 9e-6, -122.4194 + t.x[0] * 1.1e-5);
        out.push(t.to_cot_event("TRK", "a-f-G-E-V", lat, lon, t0, t0 + 10_000));
    }
    out
}
