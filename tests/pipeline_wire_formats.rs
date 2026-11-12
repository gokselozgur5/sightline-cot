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
