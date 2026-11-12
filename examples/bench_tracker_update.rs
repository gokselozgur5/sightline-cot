//! Per-frame update timing for the multi-target tracker with 10 and 100 targets.
//! Run with: cargo run --release --example bench_tracker_update

use std::time::Instant;

use sightline_cot::tracker::{BoundingBox, Detection, MultiTargetTracker};

const WARMUP_FRAMES: usize = 50;
const FRAMES: usize = 500;
const DT_S: f64 = 1.0;
const SPACING_M: f64 = 50.0;
const SPEED_MPS: f64 = 2.0;

