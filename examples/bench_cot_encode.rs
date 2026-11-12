//! Throughput benchmark for CoT 2.0 XML encoding.
//! Run with: cargo run --release --example bench_cot_encode

use std::hint::black_box;
use std::time::Instant;

use sightline_cot::cot::CotEvent;

const EVENTS: usize = 200_000;
const ROUNDS: usize = 5;
const BASE_TIME_MS: i64 = 1_767_225_600_000;

/// Builds a synthetic track event whose fields vary slightly with the index.
fn sample_event(i: usize) -> CotEvent {
    CotEvent {
        uid: format!("sightline-track-{i}"),
        cot_type: "a-h-G".to_string(),
        how: "m-r".to_string(),
        time_ms: BASE_TIME_MS + i as i64,
        stale_ms: BASE_TIME_MS + i as i64 + 60_000,
        lat_deg: 37.7749 + i as f64 * 1e-6,
        lon_deg: -122.4194 - i as f64 * 1e-6,
        hae_m: 50.0,
        ce_m: 4.5,
        le_m: 3.2,
        course_deg: 90.0,
        speed_mps: 1.5,
    }
}

