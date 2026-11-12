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
fn ground_hit(dem: &DemGrid) -> [f64; 3] {
    let intr = Intrinsics::Pinhole {
        fx: 1000.0,
        fy: 1000.0,
        cx: 960.0,
        cy: 540.0,
    };
    let pose = CameraPose {
        roll_rad: 0.0,
        pitch_rad: 0.0,
        yaw_rad: 0.0,
        cam_up_m: 1000.0,
    };
    let ray = intr.pixel_to_ray(1400.0, 540.0).expect("ray");
    dem.intersect_dem(ray, &pose, 20).expect("dem hit")
}
