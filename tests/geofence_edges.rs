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

const SQUARE: [[f64; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

#[test]
fn rejects_non_finite_and_inverted_envelope() {
    let mut poly = SQUARE;
    poly[1][0] = f64::NAN;
    assert!(Geofence::new(&poly, 0.0, 100.0).is_err());
    assert!(Geofence::new(&SQUARE, 100.0, 0.0).is_err());
}

#[test]
fn rejects_collinear_polygon() {
    let line = [[0.0, 0.0], [0.5, 0.5], [1.0, 1.0]];
    assert!(Geofence::new(&line, 0.0, 100.0).is_err());
}
