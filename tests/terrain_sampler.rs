use sightline_cot::terrain_sampler::{
    GeoTiffElevationGrid, TerrainInterceptSolver, TerrainRay, TerrainSamplerError,
};

#[test]
fn test_terrain_intercept_unconverged_max_iterations() {
    let raw = vec![0.0; 121];
    let grid = GeoTiffElevationGrid::from_raw(0.0, 0.0, 10.0, 10.0, 11, 11, &raw, None).unwrap();
    let ray = TerrainRay::new([20.0, 50.0, 50.0], [1.0, 0.0, -1.0]).unwrap();
    let solver = TerrainInterceptSolver::new(5.0, 1000.0, 1e-9, 1).unwrap();
    assert_eq!(
        solver.solve(&grid, &ray),
        Err(TerrainSamplerError::MaxIterationsReached)
    );
}
