use sightline_cot::{GeoTiffElevationGrid, TerrainInterceptSolver, TerrainRay};

#[test]
fn ray_starting_below_the_surface_is_not_reported_as_a_hit() {
    let grid = GeoTiffElevationGrid::flat(0.0, 0.0, 10.0, 10.0, 11, 11, 50.0).unwrap();
    let ray = TerrainRay::new([50.0, 50.0, 10.0], [1.0, 0.0, 0.0]).unwrap();
    let solver = TerrainInterceptSolver::default();
    match solver.solve(&grid, &ray) {
        Err(_) => {}
        Ok(hit) => {
            let surface = grid.sample_bilinear(hit.point[0], hit.point[1]).unwrap();
            assert!(
                (hit.point[2] - surface).abs() <= solver.tolerance,
                "hit at z = {} but the surface is at {}",
                hit.point[2],
                surface
            );
        }
    }
}
