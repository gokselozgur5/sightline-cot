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

/// Serializes every event and returns the total number of XML bytes produced.
fn encode_all(events: &[CotEvent]) -> usize {
    events.iter().map(|e| black_box(e.to_xml()).len()).sum()
}

fn main() {
    let events: Vec<CotEvent> = (0..EVENTS).map(sample_event).collect();
    let mut best = f64::INFINITY;
    let mut bytes = 0;
    for _ in 0..ROUNDS {
        let start = Instant::now();
        bytes = encode_all(&events);
        best = best.min(start.elapsed().as_secs_f64());
    }
    let ns_per_event = best * 1e9 / EVENTS as f64;
    let events_per_s = EVENTS as f64 / best;
    let mib_per_s = bytes as f64 / best / 1_048_576.0;
    println!("CoT 2.0 XML encode ({EVENTS} events, best of {ROUNDS} rounds)");
    println!(
        "{:<10} {:>12} {:>14} {:>12} {:>12}",
        "Events", "ns/event", "events/s", "MiB/s", "Avg bytes"
    );
    println!(
        "{EVENTS:<10} {ns_per_event:>12.1} {events_per_s:>14.0} {mib_per_s:>12.1} {:>12}",
        bytes / EVENTS
    );
}
