//! Deterministic End-to-End Latency Benchmark for Sightline CoT.
//! Simulates 10,000 frames of multi-target tracking and CoT dispatch.

use std::time::Instant;

use sightline_cot::camera::{
    enu_to_geodetic, estimate_covariance_ce_le, intersect_ground, CameraPose, Intrinsics,
};
use sightline_cot::cot::CotEvent;
use sightline_cot::tracker::{
    normalize_embedding, BoundingBox, Detection, MultiTargetTracker, Track, TrackStatus,
};

