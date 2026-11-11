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

#[test]
fn test_wgs84_antimeridian_wrap_and_equator_oracle() {
    let (_, lon_wrap, _) = enu_to_geodetic([1000.0, 0.0, 0.0], 0.0, 179.9999, 0.0);
    assert!((-180.0..=180.0).contains(&lon_wrap));
    assert!((lon_wrap - -179.99112).abs() < 1e-5);

    let (lat_n, lon_e, _) = enu_to_geodetic([1000.0, 1000.0, 0.0], 0.0, 0.0, 0.0);
    assert!((lat_n - (1000.0 / 110574.39)).abs() < 1e-6);
    assert!((lon_e - (1000.0 / 111319.49)).abs() < 1e-6);

    let (lat_ds, lon_ds, hae_ds) = enu_to_geodetic([500.0, -500.0, 0.0], 31.5, 35.5, -430.0);
    assert!(lat_ds.is_finite() && lon_ds.is_finite());
    assert!((hae_ds - -430.0).abs() < 0.1);
}
