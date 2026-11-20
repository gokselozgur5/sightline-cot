use sightline_cot::camera::{CameraPose, DemGrid};

#[test]
fn dem_intersection_is_found_on_a_slope_rising_away_from_the_camera() {
    let dep = 15.0_f64.to_radians();
    let pitch_rad = std::f64::consts::FRAC_PI_2 - dep;
    let pose = CameraPose {
        roll_rad: 0.0,
        pitch_rad,
        yaw_rad: 0.0,
        cam_up_m: 100.0,
    };
    for slope in [0.25, 0.30] {
        let elevations: Vec<f64> = (0..401).flat_map(|r| [slope * r as f64; 3]).collect();
        let grid = DemGrid::new(-1.0, 0.0, 1.0, 3, 401, elevations).unwrap();
        let want_n = 100.0 / (dep.tan() + slope);
        let hit = grid.intersect_dem([0.0, 0.0, 1.0], &pose, 64);
        let p = hit.unwrap_or_else(|e| panic!("slope {slope}: {e:?}, want north {want_n:.1}"));
        assert!(
            (p[1] - want_n).abs() < 0.5,
            "slope {slope}: north {} want {want_n:.1}",
            p[1]
        );
    }
}
