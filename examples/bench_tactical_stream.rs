//! End-to-End Tactical Mission Benchmark for Sightline CoT.
//! Simulates 10,000 frames through DEM ray intersection, Re-ID tracking,
//! MIL-STD classification, and CoT broadcast.

use std::time::Instant;

use sightline_cot::camera::{
    enu_to_geodetic, estimate_covariance_ce_le, intersect_ground, CameraPose, Intrinsics,
};
use sightline_cot::cot::CotEvent;
use sightline_cot::tracker::{
    normalize_embedding, BoundingBox, Detection, MultiTargetTracker, Track, TrackStatus,
};

const NUM_FRAMES: usize = 10_000;
const DT_S: f64 = 0.033;
const CAM_ALT_M: f64 = 600.0;
const REF_LAT: f64 = 37.7749;
const REF_LON: f64 = -122.4194;
const REF_HAE: f64 = 100.0;
const BOUND_E_M: f64 = 400.0;
const BOUND_N_M: f64 = 200.0;
const IMG_W: f64 = 1920.0;
const IMG_H: f64 = 1080.0;

#[derive(Debug, Default)]
struct LatencyStats {
    micros: Vec<f64>,
}

impl LatencyStats {
    fn with_capacity(cap: usize) -> Self {
        Self {
            micros: Vec::with_capacity(cap),
        }
    }

    fn record(&mut self, us: f64) {
        self.micros.push(us);
    }

    fn compute_metrics(&mut self) -> (f64, f64, f64, f64, f64) {
        self.micros.sort_by(|a, b| a.total_cmp(b));
        let len = self.micros.len();
        let p50 = self.micros[len * 50 / 100];
        let p99 = self.micros[len * 99 / 100];
        let max = *self.micros.last().unwrap_or(&0.0);
        let mean = self.micros.iter().sum::<f64>() / (len as f64);
        (p50, p99, max, mean, 1_000_000.0 / mean)
    }
}

struct SyntheticTarget {
    east_m: f64,
    north_m: f64,
    vel_e: f64,
    vel_n: f64,
    raw_emb: [f32; 8],
    cot_type: &'static str,
}

impl SyntheticTarget {
    fn step(&mut self, dt: f64) {
        self.east_m += self.vel_e * dt;
        self.north_m += self.vel_n * dt;
        if self.east_m.abs() > BOUND_E_M {
            self.vel_e = -self.vel_e;
        }
        if self.north_m.abs() > BOUND_N_M {
            self.vel_n = -self.vel_n;
        }
    }
}

fn init_scenario() -> Vec<SyntheticTarget> {
    vec![
        SyntheticTarget {
            east_m: 100.0,
            north_m: 150.0,
            vel_e: 12.0,
            vel_n: 8.0,
            raw_emb: [1.0, 0.2, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0],
            cot_type: "a-f-G-E-V-C",
        },
        SyntheticTarget {
            east_m: -80.0,
            north_m: 100.0,
            vel_e: -5.0,
            vel_n: 15.0,
            raw_emb: [0.0, 1.0, 0.3, 0.0, 0.0, 0.0, 0.0, 0.0],
            cot_type: "a-f-G-E-V",
        },
        SyntheticTarget {
            east_m: 50.0,
            north_m: -100.0,
            vel_e: 2.0,
            vel_n: -10.0,
            raw_emb: [0.2, 0.0, 1.0, 0.5, 0.0, 0.0, 0.0, 0.0],
            cot_type: "a-f-G-E-S",
        },
    ]
}

struct Observation {
    u: f64,
    v: f64,
    emb: [f32; 8],
    cot_type: &'static str,
}

fn observe(tgt: &SyntheticTarget, cam: &Intrinsics) -> Option<Observation> {
    let Intrinsics::Pinhole { fx, fy, cx, cy } = *cam else {
        panic!("bench supports only the pinhole camera model");
    };
    let u = fx * tgt.east_m / CAM_ALT_M + cx;
    let v = cy - fy * tgt.north_m / CAM_ALT_M;
    if !(0.0..IMG_W).contains(&u) || !(0.0..IMG_H).contains(&v) {
        return None;
    }
    Some(Observation {
        u,
        v,
        emb: normalize_embedding(&tgt.raw_emb),
        cot_type: tgt.cot_type,
    })
}

fn pregenerate(targets: &mut [SyntheticTarget], cam: &Intrinsics) -> Vec<Vec<Observation>> {
    let mut frames = Vec::with_capacity(NUM_FRAMES);
    for _ in 0..NUM_FRAMES {
        let mut obs = Vec::with_capacity(targets.len());
        for tgt in targets.iter_mut() {
            tgt.step(DT_S);
            obs.extend(observe(tgt, cam));
        }
        frames.push(obs);
    }
    frames
}

fn dem_elevation_lookup(east_m: f64, north_m: f64) -> f64 {
    5.0 * (east_m * 0.01).sin() * (north_m * 0.01).cos()
}

fn detect(o: &Observation, cam: &Intrinsics, pose: &CameraPose) -> Option<Detection> {
    let ray = cam.pixel_to_ray(o.u, o.v).ok()?;
    let mut ground_elev = 0.0f64;
    let mut ground_pt = [0.0; 3];
    for _ in 0..3 {
        ground_pt = intersect_ground(ray, pose, ground_elev).ok()?;
        ground_elev = dem_elevation_lookup(ground_pt[0], ground_pt[1]);
    }
    let (ce90, le90) = estimate_covariance_ce_le(cam, pose, o.u, o.v, ground_elev, 1.0, 0.001).ok()?;
    Some(Detection {
        east_m: ground_pt[0],
        north_m: ground_pt[1],
        hae_m: ground_pt[2],
        ce90_m: ce90,
        le90_m: le90,
        bbox: BoundingBox {
