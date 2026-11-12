use sightline_cot::{Geofence, GeofenceState};

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

#[test]
fn accepts_concave_polygon() {
    let poly = [[0.0, 0.0], [0.0, 3.0], [3.0, 3.0], [1.0, 1.5], [3.0, 0.0]];
    assert!(Geofence::new(&poly, 0.0, 100.0).is_ok());
}

fn tiny_square() -> Geofence {
    let poly = [[0.0, 0.0], [0.0, 0.0002], [0.0002, 0.0002], [0.0002, 0.0]];
    Geofence::new(&poly, 0.0, 1000.0).unwrap()
}

#[test]
fn hysteresis_holds_state_within_margin() {
    let gf = tiny_square();
    let held = gf.evaluate_breach_hysteresis(0.0001, -0.0001, 10.0, Some(true), 50.0);
    assert_eq!(held.state, GeofenceState::Inside);
    let held = gf.evaluate_breach_hysteresis(0.0001, 0.0001, 10.0, Some(false), 50.0);
    assert_eq!(held.state, GeofenceState::Outside);
}

#[test]
fn hysteresis_transitions_beyond_margin() {
    let gf = tiny_square();
    let b = gf.evaluate_breach_hysteresis(0.0001, -0.0001, 10.0, Some(true), 5.0);
    assert_eq!(b.state, GeofenceState::Exited);
    let b = gf.evaluate_breach_hysteresis(0.0001, 0.0001, 10.0, Some(false), 5.0);
    assert_eq!(b.state, GeofenceState::Entered);
}

#[test]
fn hysteresis_without_history_matches_plain_evaluation() {
    let gf = tiny_square();
    let b = gf.evaluate_breach_hysteresis(0.0001, 0.0001, 10.0, None, 50.0);
    assert_eq!(b.state, GeofenceState::Inside);
}

#[test]
fn antimeridian_polygon_containment() {
    let poly = [[0.0, 179.0], [0.0, -179.0], [1.0, -179.0], [1.0, 179.0]];
    let gf = Geofence::new(&poly, 0.0, 1000.0).unwrap();
    assert!(gf.is_point_inside_polygon(0.5, 179.5));
    assert!(gf.is_point_inside_polygon(0.5, -179.5));
    assert!(!gf.is_point_inside_polygon(0.5, 0.0));
}

#[test]
fn concave_polygon_notch_is_outside() {
    let poly = [[0.0, 0.0], [0.0, 3.0], [3.0, 3.0], [3.0, 2.0], [1.0, 1.5], [3.0, 1.0], [3.0, 0.0]];
    let gf = Geofence::new(&poly, 0.0, 1000.0).unwrap();
    assert!(gf.is_point_inside_polygon(0.5, 1.5));
    assert!(!gf.is_point_inside_polygon(2.5, 1.5));
}
