//! Deterministic End-to-End Latency Benchmark for Sightline CoT.
//! Simulates 10,000 frames of multi-target tracking and CoT dispatch.

use std::time::Instant;

use sightline_cot::camera::{
    enu_to_geodetic, estimate_covariance_ce_le, intersect_ground, CameraPose, Intrinsics,
};
use sightline_cot::cot::CotEvent;
use sightline_cot::tracker::{
    normalize_embedding, BoundingBox, Detection, MultiTargetTracker, Track, TrackStatus,
};

const NUM_FRAMES: usize = 10_000;
const DT_S: f64 = 0.033;
const CAM_ALT_M: f64 = 500.0;
const REF_LAT: f64 = 37.7749;
const REF_LON: f64 = -122.4194;
const REF_HAE: f64 = 50.0;
const BOUND_E_M: f64 = 400.0;
const BOUND_N_M: f64 = 200.0;
const IMG_W: f64 = 1920.0;
const IMG_H: f64 = 1080.0;

#[derive(Debug, Default)]
struct LatencyStats {
    micros: Vec<f64>,
}

impl LatencyStats {
    fn with_capacity(cap: usize) -> Self {
        Self {
            micros: Vec::with_capacity(cap),
        }
    }

    fn record(&mut self, us: f64) {
        self.micros.push(us);
    }

    fn compute_metrics(&mut self) -> (f64, f64, f64, f64) {
        self.micros.sort_by(|a, b| a.total_cmp(b));
        let len = self.micros.len();
        let p50 = self.micros[len * 50 / 100];
        let p99 = self.micros[len * 99 / 100];
        let mean = self.micros.iter().sum::<f64>() / (len as f64);
        (p50, p99, mean, 1_000_000.0 / mean)
    }
}

