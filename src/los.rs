//! 3D Line-of-Sight (LOS) terrain masking and horizon analysis.

use crate::camera::DemGrid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position3D {
    pub east_m: f64,
    pub north_m: f64,
    pub up_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LosResult {
    pub is_visible: bool,
    pub obstruction: Option<Position3D>,
    pub min_clearance_m: f64,
}

pub const WGS84_EARTH_RADIUS_M: f64 = 6_371_000.0;

pub fn earth_curvature_drop(distance_m: f64) -> f64 {
    if distance_m <= 0.0 {
        0.0
    } else {
        distance_m * distance_m / (2.0 * WGS84_EARTH_RADIUS_M)
    }
}

fn sample_point(obs: Position3D, target: Position3D, frac: f64) -> (f64, f64, f64) {
    (
        obs.east_m + frac * (target.east_m - obs.east_m),
        obs.north_m + frac * (target.north_m - obs.north_m),
        obs.up_m + frac * (target.up_m - obs.up_m),
    )
}
