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
    let points = sample_points(POINTS);
    let utms: Vec<UtmCoord> = points
        .iter()
        .filter_map(|&(lat, lon)| wgs84_to_utm(lat, lon).ok())
        .collect();

    let fwd_s = best_seconds(|| {
        for &(lat, lon) in &points {
            black_box(wgs84_to_utm(lat, lon).ok());
        }
    });
    let inv_s = best_seconds(|| {
        for utm in &utms {
            black_box(utm_to_wgs84(black_box(utm)).ok());
        }
    });
    let mgrs_s = best_seconds(|| {
        for &(lat, lon) in &points {
            black_box(format_mgrs(lat, lon, MGRS_DIGITS).ok());
        }
    });

