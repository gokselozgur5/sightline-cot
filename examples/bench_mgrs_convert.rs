//! Throughput benchmark for WGS84, UTM and MGRS conversions.
//! Run with: cargo run --release --example bench_mgrs_convert

use std::hint::black_box;
use std::time::Instant;

use sightline_cot::mgrs::{format_mgrs, utm_to_wgs84, wgs84_to_utm, UtmCoord};

const POINTS: usize = 100_000;
const ROUNDS: usize = 5;
const MGRS_DIGITS: usize = 5;

