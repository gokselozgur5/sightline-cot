use sightline_cot::Geofence;

fn unit_square() -> Geofence {
    let poly = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
    Geofence::new(&poly, 0.0, 1000.0).unwrap()
}

#[test]
fn distance_uses_edge_interior_not_only_vertices() {
    let d = unit_square().distance_to_boundary_m(0.5, -0.001);
    assert!(d < 120.0, "got {d}");
}

#[test]
fn distance_at_vertex_is_zero() {
    assert!(unit_square().distance_to_boundary_m(0.0, 0.0) < 1e-6);
}
