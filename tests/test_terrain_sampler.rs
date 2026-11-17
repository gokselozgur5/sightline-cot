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

    let aspect = grid.aspect_deg(5.0, 5.0).unwrap();
    // West facing downhill: aspect is 270 degrees
    assert!((aspect - 270.0).abs() < 1.0);
}

#[test]
fn test_ray_validation_and_point_at() {
    assert!(TerrainRay::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]).is_err());
    assert!(TerrainRay::new([f64::NAN, 0.0, 0.0], [1.0, 0.0, 0.0]).is_err());

    let ray = TerrainRay::new([10.0, 20.0, 30.0], [0.0, 0.0, -2.0]).unwrap();
    assert_eq!(ray.direction, [0.0, 0.0, -1.0]);
    let pt = ray.point_at(10.0);
    assert_eq!(pt, [10.0, 20.0, 20.0]);
}

#[test]
fn test_terrain_intercept_sloped_surface() {
    // Plane: z = 0.5 * x
    let mut raw = Vec::new();
    for _r in 0..11 {
        for c in 0..11 {
            raw.push(c as f64 * 5.0);
        }
    }
    let grid = GeoTiffElevationGrid::from_raw(0.0, 0.0, 10.0, 10.0, 11, 11, &raw, None).unwrap();

    // Slanted ray pointing downward at 45 degrees toward x=50, y=50
    let ray = TerrainRay::new([20.0, 50.0, 50.0], [1.0, 0.0, -1.0]).unwrap();
    let solver = TerrainInterceptSolver::new(5.0, 1000.0, 0.01, 100).unwrap();
    let intercept = solver.solve(&grid, &ray).unwrap();

    assert!(intercept.point[0] >= 0.0 && intercept.point[0] <= 100.0);
    assert!((intercept.point[1] - 50.0).abs() < 0.1);
    let terrain_h = grid
        .sample_bilinear(intercept.point[0], intercept.point[1])
        .unwrap();
    assert!((intercept.point[2] - terrain_h).abs() < 0.05);
}

#[test]
fn test_ray_pointing_away_error() {
    let grid = GeoTiffElevationGrid::flat(0.0, 0.0, 10.0, 10.0, 5, 5, 100.0).unwrap();
    let ray = TerrainRay::new([20.0, 20.0, 200.0], [0.0, 0.0, 1.0]).unwrap();
    let solver = TerrainInterceptSolver::default();
    let res = solver.solve(&grid, &ray);
    assert_eq!(res, Err(TerrainSamplerError::RayPointsAway));
