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

fn main() {
    let events: Vec<TakCotEvent> = (0..MESSAGES).map(sample_event).collect();
    let encoded: Vec<Vec<u8>> = events
        .iter()
        .map(|e| {
            let mut buf = Vec::new();
            e.encode_protobuf(&mut buf).expect("encode failed");
            buf
        })
        .collect();
    let wire_bytes: usize = encoded.iter().map(Vec::len).sum();

    let encode_s = best_seconds(|| {
        let mut out = Vec::with_capacity(256);
        for e in &events {
            out.clear();
            e.encode_protobuf(&mut out).expect("encode failed");
            black_box(out.len());
        }
    });
    let decode_s = best_seconds(|| {
        for buf in &encoded {
            let evt = TakCotEvent::decode_protobuf(black_box(buf)).expect("decode failed");
            black_box(evt);
        }
    });

    println!("TAK protobuf codec ({MESSAGES} messages, best of {ROUNDS} rounds)");
    println!(
        "{:<10} {:>12} {:>14} {:>12}",
        "Operation", "ns/msg", "msgs/s", "MiB/s"
    );
    for (name, secs) in [("encode", encode_s), ("decode", decode_s)] {
        let ns_per_msg = secs * 1e9 / MESSAGES as f64;
        let msgs_per_s = MESSAGES as f64 / secs;
        let mib_per_s = wire_bytes as f64 / secs / 1_048_576.0;
        println!("{name:<10} {ns_per_msg:>12.1} {msgs_per_s:>14.0} {mib_per_s:>12.1}");
    }
}
