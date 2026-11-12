//! Throughput benchmark for CoT 2.0 XML encoding.
//! Run with: cargo run --release --example bench_cot_encode

use std::hint::black_box;
use std::time::Instant;

use sightline_cot::cot::CotEvent;

const EVENTS: usize = 200_000;
const ROUNDS: usize = 5;
const BASE_TIME_MS: i64 = 1_767_225_600_000;

