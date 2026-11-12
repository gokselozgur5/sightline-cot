//! Per-frame update timing for the multi-target tracker with 10 and 100 targets.
//! Run with: cargo run --release --example bench_tracker_update

use std::time::Instant;

use sightline_cot::tracker::{BoundingBox, Detection, MultiTargetTracker};

const WARMUP_FRAMES: usize = 50;
const FRAMES: usize = 500;
const DT_S: f64 = 1.0;
const SPACING_M: f64 = 50.0;
const SPEED_MPS: f64 = 2.0;

/// Builds one frame of synthetic detections on a grid, all moving east at a constant speed.
fn synthetic_detections(count: usize, frame: usize) -> Vec<Detection> {
    (0..count)
        .map(|i| {
            let mut embedding = [0.0_f32; 8];
            embedding[i % 8] = 1.0;
            let x1 = (i * 10) as f64;
            Detection {
                east_m: (i % 10) as f64 * SPACING_M + SPEED_MPS * frame as f64,
                north_m: (i / 10) as f64 * SPACING_M,
                hae_m: 0.0,
                ce90_m: 5.0,
                le90_m: 5.0,
                bbox: BoundingBox {
                    x1,
                    y1: 0.0,
                    x2: x1 + 8.0,
                    y2: 16.0,
                },
                embedding,
            }
        })
        .collect()
}

/// Returns the mean microseconds per tracker step and the resulting track count.
fn time_updates(count: usize) -> (f64, usize) {
    let frames: Vec<Vec<Detection>> = (0..WARMUP_FRAMES + FRAMES)
        .map(|f| synthetic_detections(count, f))
        .collect();
    let mut tracker = MultiTargetTracker::new(3, 5, 2.0);
    for dets in &frames[..WARMUP_FRAMES] {
        tracker.step(DT_S, dets).expect("warmup step failed");
    }
    let start = Instant::now();
    for dets in &frames[WARMUP_FRAMES..] {
        tracker.step(DT_S, dets).expect("step failed");
    }
    let us_per_frame = start.elapsed().as_secs_f64() * 1e6 / FRAMES as f64;
    (us_per_frame, tracker.tracks.len())
}

