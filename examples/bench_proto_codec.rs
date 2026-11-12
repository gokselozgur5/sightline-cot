//! Throughput benchmark for TAK protobuf encode and decode.
//! Run with: cargo run --release --example bench_proto_codec

use std::hint::black_box;
use std::time::Instant;

use sightline_cot::proto::TakCotEvent;

const MESSAGES: usize = 200_000;
const ROUNDS: usize = 5;
const BASE_TIME_MS: u64 = 1_767_225_600_000;

