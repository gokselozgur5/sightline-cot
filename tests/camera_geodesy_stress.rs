use sightline_cot::camera::{
    enu_to_geodetic, intersect_ground, CameraError, CameraPose, Intrinsics,
};
use std::f64::consts::PI;

#[test]
fn test_wgs84_extreme_coordinates_accuracy() {
    let origins = [
        (0.0, 0.0, 0.0),
        (0.0, 179.9999, 100.0),
        (0.0, -179.9999, 100.0),
        (89.9999, 0.0, 500.0),
        (-89.9999, 0.0, 500.0),
        (31.5, 35.5, -430.0),
    ];
    for (lat0, lon0, hae0) in origins {
        let (lat_z, lon_z, hae_z) = enu_to_geodetic([0.0, 0.0, 0.0], lat0, lon0, hae0);
        assert!((lat_z - lat0).abs() < 1e-7);
        assert!((lon_z - lon0).abs() < 1e-7);
        assert!((hae_z - hae0).abs() < 1e-4);

        let (lat_up, lon_up, hae_up) = enu_to_geodetic([0.0, 0.0, 100.0], lat0, lon0, hae0);
        assert!((lat_up - lat0).abs() < 1e-6);
        assert!((lon_up - lon0).abs() < 1e-6);
        assert!((hae_up - (hae0 + 100.0)).abs() < 1e-3);
    }
}
