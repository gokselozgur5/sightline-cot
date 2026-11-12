//! Throughput benchmark for WGS84, UTM and MGRS conversions.
//! Run with: cargo run --release --example bench_mgrs_convert

use std::hint::black_box;
use std::time::Instant;

use sightline_cot::mgrs::{format_mgrs, utm_to_wgs84, wgs84_to_utm, UtmCoord};

const POINTS: usize = 100_000;
const ROUNDS: usize = 5;
const MGRS_DIGITS: usize = 5;

/// Spreads points over the MGRS latitude band and most of the longitude range.
fn sample_points(count: usize) -> Vec<(f64, f64)> {
    (0..count)
        .map(|i| {
            let lat = -60.0 + 120.0 * (i as f64 * 0.618_033_988_75).fract();
            let lon = -170.0 + 340.0 * (i as f64 * 0.414_213_562_37).fract();
            (lat, lon)
        })
        .collect()
}

