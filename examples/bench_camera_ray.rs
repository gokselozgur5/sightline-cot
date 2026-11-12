//! Throughput benchmark for camera ray casting.
//! Run with: cargo run --release --example bench_camera_ray

use std::hint::black_box;
use std::time::Instant;

use sightline_cot::camera::{intersect_ground, CameraPose, Intrinsics};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const STEP: usize = 4;
const ROUNDS: usize = 5;

/// Runs the workload several times and prints the best-of-rounds timing.
fn report<F: FnMut() -> usize>(label: &str, mut run: F) {
    let mut best = f64::INFINITY;
    let mut rays = 0;
    for _ in 0..ROUNDS {
        let start = Instant::now();
        rays = run();
        best = best.min(start.elapsed().as_secs_f64());
    }
    let ns_per_ray = best * 1e9 / rays as f64;
    let rays_per_s = rays as f64 / best;
    println!("{label:<32} {rays:>10} {ns_per_ray:>12.1} {rays_per_s:>14.0}");
}

