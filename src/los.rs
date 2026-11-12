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

fn check_step(dem: &DemGrid, p: (f64, f64, f64), dist: f64) -> (f64, Option<Position3D>) {
    let (e, n, up) = p;
    let eff_alt = up - earth_curvature_drop(dist);
    if let Some(h) = dem.elevation_at(e, n) {
        let clr = eff_alt - h;
        if clr < 0.0 {
            return (
                clr,
                Some(Position3D {
                    east_m: e,
                    north_m: n,
                    up_m: h,
                }),
            );
        }
        (clr, None)
    } else {
        (f64::MAX, None)
    }
}
