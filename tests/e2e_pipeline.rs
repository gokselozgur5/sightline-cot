//! Deterministic End-to-End Multi-Target Pixel-to-CoT Integration Suite.

use sightline_cot::camera::{enu_to_geodetic, estimate_covariance_ce_le, intersect_ground};
use sightline_cot::camera::{CameraPose, Intrinsics};
use sightline_cot::cot::{CotEvent, CotUdpDispatcher};
use sightline_cot::tracker::{BoundingBox, Detection, MultiTargetTracker, Track, TrackStatus};
use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::Duration;
