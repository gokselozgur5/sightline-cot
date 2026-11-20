use sightline_cot::{Geofence, GeofenceState};

#[test]
fn altitude_exit_is_not_held_by_the_horizontal_margin() {
    let square = [[37.0, -122.0], [37.0, -121.0], [38.0, -121.0], [38.0, -122.0]];
    let fence = Geofence::new(&square, 0.0, 1000.0).unwrap();
    let raw = fence.evaluate_breach(37.5, -121.9999, 5000.0, Some(true));
    assert_eq!(raw.state, GeofenceState::Exited);
    let held = fence.evaluate_breach_hysteresis(37.5, -121.9999, 5000.0, Some(true), 50.0);
    assert_eq!(held.state, GeofenceState::Exited);
}
