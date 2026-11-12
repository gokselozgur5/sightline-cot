//! Terrain masking of a tracked target localized on a DEM.

use sightline_cot::camera::{CameraPose, DemGrid, Intrinsics};
use sightline_cot::los::{check_los, LosResult, Position3D};
use sightline_cot::tracker::{BoundingBox, Detection, MultiTargetTracker};

fn dem(ridge_m: f64) -> DemGrid {
    let mut z = vec![0.0; 121];
    for r in 0..11 {
        z[r * 11 + 5] = ridge_m;
    }
    DemGrid::new(-500.0, -500.0, 100.0, 11, 11, z).expect("dem")
}
