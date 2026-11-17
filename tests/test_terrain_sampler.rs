use sightline_cot::terrain_sampler::{
    cot_target_from_intercept, GeoTiffElevationGrid, InterpolationMethod, TerrainInterceptSolver,
    TerrainRay, TerrainSamplerError,
};

#[test]
fn test_geotiff_grid_validation_errors() {
    assert!(GeoTiffElevationGrid::new(f64::NAN, 0.0, 1.0, 1.0, 2, 2, vec![Some(0.0); 4]).is_err());
    assert!(GeoTiffElevationGrid::new(0.0, 0.0, -1.0, 1.0, 2, 2, vec![Some(0.0); 4]).is_err());
    assert!(GeoTiffElevationGrid::new(0.0, 0.0, 1.0, 1.0, 1, 2, vec![Some(0.0); 2]).is_err());
    assert!(GeoTiffElevationGrid::new(0.0, 0.0, 1.0, 1.0, 2, 2, vec![Some(0.0); 5]).is_err());
}

#[test]
fn test_geotiff_from_raw_with_nodata() {
    let raw = vec![-9999.0, 100.0, 200.0, -9999.0];
    let grid =
        GeoTiffElevationGrid::from_raw(0.0, 0.0, 10.0, 10.0, 2, 2, &raw, Some(-9999.0)).unwrap();
    assert_eq!(grid.get_node(0, 0), None);
    assert_eq!(grid.get_node(1, 0), Some(100.0));
    assert_eq!(grid.get_node(0, 1), Some(200.0));
    assert_eq!(grid.get_node(1, 1), None);
    assert_eq!(grid.min_max_elevation(), Some((100.0, 200.0)));
}

#[test]
fn test_interpolation_modes() {
    let raw = vec![0.0, 10.0, 20.0, 30.0];
    let grid = GeoTiffElevationGrid::from_raw(0.0, 0.0, 10.0, 10.0, 2, 2, &raw, None).unwrap();

    let linear_val = grid
        .sample(5.0, 5.0, InterpolationMethod::Bilinear)
        .unwrap();
    assert!((linear_val - 15.0).abs() < 1e-6);

    let nearest_val = grid
        .sample(1.0, 1.0, InterpolationMethod::NearestNeighbor)
        .unwrap();
    assert_eq!(nearest_val, 0.0);
}

#[test]
fn test_aspect_and_slope() {
    // East-facing slope: height increases with x
    let raw = vec![0.0, 10.0, 0.0, 10.0];
    let grid = GeoTiffElevationGrid::from_raw(0.0, 0.0, 10.0, 10.0, 2, 2, &raw, None).unwrap();
    let slope = grid.slope_deg(5.0, 5.0).unwrap();
    assert!((slope - 45.0).abs() < 0.5);
