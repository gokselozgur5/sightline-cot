use sightline_cot::camera::DemGrid;
use sightline_cot::elevation_mask::{ObstaclePoint, ObstacleProfile};
use sightline_cot::los::{check_los, earth_curvature_drop, Position3D};

#[test]
fn test_target_under_terrain_is_blocked() {
    let elev = vec![0.0, 0.0, 0.0, 0.0, 20.0, 0.0, 0.0, 0.0, 0.0];
    let dem = DemGrid::new(0.0, 0.0, 10.0, 3, 3, elev).unwrap();
    let obs = Position3D {
        east_m: 0.0,
        north_m: 10.0,
        up_m: 1000.0,
    };
    let tgt = Position3D {
        east_m: 10.0,
        north_m: 10.0,
        up_m: 5.0,
    };
    let res = check_los(&dem, obs, tgt, 10);
    assert!(!res.is_visible);
    assert!(res.min_clearance_m < 0.0);
}

#[test]
fn test_curvature_drop_consistency_across_modules() {
    let dist_m = 20_000.0;
    let los_drop = earth_curvature_drop(dist_m);

    let profile = ObstacleProfile::new(vec![ObstaclePoint {
        distance_m: dist_m,
        elevation_m: 0.0,
        height_agl_m: 0.0,
    }])
    .unwrap();

    let mask_angle = profile.compute_mask_angle_deg(0.0, true).unwrap();
    let implied_drop = -dist_m * mask_angle.to_radians().tan();

    assert!(
        (los_drop - implied_drop).abs() < 1e-3,
        "Curvature drop mismatch at 20km: los={} vs mask={}",
        los_drop,
        implied_drop
    );
}
