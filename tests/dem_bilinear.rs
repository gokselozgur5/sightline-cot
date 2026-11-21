use sightline_cot::dem_bilinear::{sample_dem_bilinear, DemError, DemRaster};

#[test]
fn test_exact_at_nodes() {
    let mut elev = Vec::with_capacity(9);
    for i in 1..=9 {
        elev.push(Some(i as f64 * 10.0));
    }
    let dem = DemRaster::new(100.0, 200.0, 10.0, 3, 3, elev).unwrap();
    for r in 0..3 {
        for c in 0..3 {
            let (e, n) = (100.0 + c as f64 * 10.0, 200.0 + r as f64 * 10.0);
            let expected = (r * 3 + c + 1) as f64 * 10.0;
            assert_eq!(dem.sample(e, n), Some(expected));
            assert_eq!(sample_dem_bilinear(&dem, e, n), Some(expected));
        }
    }
}

#[test]
fn test_linear_between_and_planar_surface() {
    let elev = vec![Some(10.0), Some(30.0), Some(50.0), Some(70.0)];
    let dem = DemRaster::new(0.0, 0.0, 10.0, 2, 2, elev).unwrap();
    assert!((dem.sample(5.0, 0.0).unwrap() - 20.0).abs() < 1e-9);
    assert!((dem.sample(0.0, 5.0).unwrap() - 30.0).abs() < 1e-9);
    assert!((dem.sample(5.0, 5.0).unwrap() - 40.0).abs() < 1e-9);
    let (e, n) = (3.5, 7.2);
    let expected = 10.0 + 2.0 * e + 4.0 * n;
    assert!((dem.sample(e, n).unwrap() - expected).abs() < 1e-9);
}

#[test]
fn test_nodata_corner_returns_none() {
    let mut elev = vec![Some(10.0); 9];
    elev[4] = None;
    let dem = DemRaster::new(0.0, 0.0, 10.0, 3, 3, elev).unwrap();
    assert_eq!(dem.sample(5.0, 5.0), None);
    assert_eq!(dem.sample(15.0, 5.0), None);
    assert_eq!(dem.sample(5.0, 15.0), None);
    assert_eq!(dem.sample(15.0, 15.0), None);
}

#[test]
fn test_nodata_from_raw_and_out_of_bounds() {
    let raw = [10.0, -9999.0, f64::NAN, 40.0];
    let dem = DemRaster::from_f64(0.0, 0.0, 5.0, 2, 2, &raw, Some(-9999.0)).unwrap();
    assert_eq!(dem.get_node(1, 0), None);
    assert_eq!(dem.get_node(0, 1), None);
    assert_eq!(dem.sample(2.5, 2.5), None);
    assert_eq!(dem.sample(-1.0, 2.0), None);
    assert_eq!(dem.sample(2.0, 10.0), None);
    assert_eq!(dem.sample(f64::NAN, 2.0), None);
}

#[test]
fn test_validation_errors_and_camera_grid_conversion() {
    let d = vec![None; 4];
    assert_eq!(
        DemRaster::new(0.0, 0.0, 0.0, 2, 2, d.clone()),
        Err(DemError::InvalidStep)
    );
    assert_eq!(
        DemRaster::new(0.0, 0.0, 1.0, 1, 2, d.clone()),
        Err(DemError::InvalidDimensions)
    );
    assert_eq!(
        DemRaster::new(0.0, 0.0, 1.0, 2, 2, vec![None; 3]),
        Err(DemError::DataLengthMismatch)
    );
    let raw = vec![10.0, 20.0, 30.0, 40.0];
    let cam = sightline_cot::camera::DemGrid::new(0.0, 0.0, 10.0, 2, 2, raw).unwrap();
    let raster = DemRaster::from(&cam);
    assert_eq!(raster.sample(5.0, 5.0), cam.elevation_at(5.0, 5.0));
}

#[test]
fn test_dimension_multiplication_overflow_rejected() {
    let cols = 1usize << 33;
    let rows = 1usize << 31;
    let err = DemRaster::new(0.0, 0.0, 1.0, cols, rows, vec![]).unwrap_err();
    assert!(
        err == DemError::DataLengthMismatch || err == DemError::InvalidDimensions,
        "unexpected error: {err:?}"
    );
}
