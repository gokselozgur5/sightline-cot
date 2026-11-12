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
            x1: o.u - 16.0,
            y1: o.v - 16.0,
            x2: o.u + 16.0,
            y2: o.v + 16.0,
        },
        embedding: o.emb,
    })
}

fn track_to_cot(track: &Track, frame_idx: usize, le_m: f64, cot_type: &str) -> CotEvent {
    let enu = [track.x[0], track.x[1], track.hae_m];
    let (lat, lon, hae) = enu_to_geodetic(enu, REF_LAT, REF_LON, REF_HAE);
    let speed = (track.x[2].powi(2) + track.x[3].powi(2)).sqrt();
    let course = track.x[2].atan2(track.x[3]).to_degrees().rem_euclid(360.0);
    let ce_m = 2.146 * (0.5 * (track.p[0][0] + track.p[1][1])).sqrt();
    let now_ms = (frame_idx as i64) * 33;
    CotEvent {
        uid: format!("track-{}", track.id),
        cot_type: cot_type.to_string(),
        how: "m-g".to_string(),
        time_ms: now_ms,
        stale_ms: now_ms + 10_000,
        lat_deg: lat,
        lon_deg: lon,
        hae_m: hae,
        ce_m,
        le_m,
        course_deg: course,
        speed_mps: speed,
    }
}

fn process_frame(
    frame_idx: usize,
    obs: &[Observation],
    cam: &Intrinsics,
    pose: &CameraPose,
    tracker: &mut MultiTargetTracker,
) -> usize {
    let dets: Vec<Detection> = obs.iter().filter_map(|o| detect(o, cam, pose)).collect();
    let le_m = dets.iter().map(|d| d.le90_m).fold(0.0, f64::max);
    tracker.step(DT_S, &dets).expect("tracker step failed");
    let mut bytes = 0;
    for (i, trk) in tracker.tracks.iter().enumerate() {
        if trk.status == TrackStatus::Confirmed {
            let cot_type = if i < obs.len() { obs[i].cot_type } else { "a-f-G-E-V" };
            bytes += track_to_cot(trk, frame_idx, le_m, cot_type).to_xml().len();
        }
    }
    bytes
}

fn main() {
    let cam = Intrinsics::Pinhole {
        fx: 1000.0,
        fy: 1000.0,
        cx: 960.0,
        cy: 540.0,
    };
    let pose = CameraPose {
        roll_rad: 0.0,
        pitch_rad: 0.0,
        yaw_rad: 0.0,
        cam_up_m: CAM_ALT_M,
    };
    let mut targets = init_scenario();
    let frames = pregenerate(&mut targets, &cam);
    let mut tracker = MultiTargetTracker::default();
    let mut stats = LatencyStats::with_capacity(NUM_FRAMES);
    let mut total_bytes = 0usize;
    for (frame_idx, obs) in frames.iter().enumerate() {
        let t0 = Instant::now();
        total_bytes += process_frame(frame_idx, obs, &cam, &pose, &mut tracker);
        stats.record(t0.elapsed().as_secs_f64() * 1_000_000.0);
    }
    let (p50, p99, max, mean, fps) = stats.compute_metrics();
    println!("=== Sightline CoT Tactical Mission Benchmark ===");
    println!("Frames processed : {NUM_FRAMES}");
    println!("Total CoT output : {total_bytes} bytes");
    println!("Latency p50      : {p50:.2} us");
    println!("Latency p99      : {p99:.2} us");
    println!("Latency max      : {max:.2} us");
    println!("Latency Mean     : {mean:.2} us");
    println!("Throughput       : {fps:.1} fps");
    verify_tracks(&tracker, &targets);
}

fn verify_tracks(tracker: &MultiTargetTracker, targets: &[SyntheticTarget]) {
    let confirmed: Vec<&Track> = tracker
        .tracks
        .iter()
        .filter(|t| t.status == TrackStatus::Confirmed)
        .collect();
    println!("Confirmed tracks : {}", confirmed.len());
    assert_eq!(confirmed.len(), targets.len(), "confirmed track count");
    for tgt in targets {
        let err = confirmed
            .iter()
            .map(|t| (t.x[0] - tgt.east_m).hypot(t.x[1] - tgt.north_m))
            .fold(f64::MAX, f64::min);
