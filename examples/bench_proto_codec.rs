//! Throughput benchmark for TAK protobuf encode and decode.
//! Run with: cargo run --release --example bench_proto_codec

use std::hint::black_box;
use std::time::Instant;

use sightline_cot::proto::TakCotEvent;

const MESSAGES: usize = 200_000;
const ROUNDS: usize = 5;
const BASE_TIME_MS: u64 = 1_767_225_600_000;

/// Builds a synthetic track message whose fields vary slightly with the index.
fn sample_event(i: usize) -> TakCotEvent {
    TakCotEvent {
        r#type: "a-h-G".to_string(),
        uid: format!("sightline-track-{i}"),
        send_time: BASE_TIME_MS + i as u64,
        start_time: BASE_TIME_MS + i as u64,
        stale_time: BASE_TIME_MS + i as u64 + 60_000,
        how: "m-r".to_string(),
        lat: 37.7749 + i as f64 * 1e-6,
        lon: -122.4194 - i as f64 * 1e-6,
        hae: 50.0,
        ce: 4.5,
        le: 3.2,
    }
}

/// Runs the workload each round and returns the best wall-clock time in seconds.
fn best_seconds<F: FnMut()>(mut run: F) -> f64 {
    let mut best = f64::INFINITY;
    for _ in 0..ROUNDS {
        let start = Instant::now();
        run();
        best = best.min(start.elapsed().as_secs_f64());
    }
    best
}

