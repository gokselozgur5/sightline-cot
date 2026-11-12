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
fn tracked_target(dem: &DemGrid) -> Position3D {
    let g = ground_hit(dem);
    let bbox = BoundingBox {
        x1: g[0] - 5.0,
        y1: g[1] - 5.0,
        x2: g[0] + 5.0,
        y2: g[1] + 5.0,
    };
    let d = Detection {
        east_m: g[0],
        north_m: g[1],
        hae_m: g[2],
        ce90_m: 1.0,
        le90_m: 1.0,
        bbox,
        embedding: [1.0; 8],
    };
    let mut tracker = MultiTargetTracker::new(2, 5, 1.0);
    tracker.step(0.1, std::slice::from_ref(&d)).expect("step");
    let t = &tracker.tracks[0];
    Position3D {
        east_m: t.x[0],
        north_m: t.x[1],
        up_m: t.hae_m + 2.0,
    }
}
fn los_from_observer(dem: &DemGrid) -> LosResult {
    let obs = Position3D {
        east_m: -400.0,
        north_m: 0.0,
        up_m: 30.0,
    };
    check_los(dem, obs, tracked_target(dem), 200)
}
#[test]
fn ridge_masks_tracked_target() {
    let r = los_from_observer(&dem(100.0));
    assert!(!r.is_visible);
    let o = r.obstruction.expect("obstruction");
    assert!(o.east_m.abs() < 100.0, "obstruction near the ridge line");
    assert!(r.min_clearance_m < 0.0);
}
#[test]
fn flat_terrain_reveals_tracked_target() {
    let r = los_from_observer(&dem(0.0));
    assert!(r.is_visible);
    assert!(r.obstruction.is_none());
    assert!(r.min_clearance_m > 0.0);
}
