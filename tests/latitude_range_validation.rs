use sightline_cot::{wgs84_to_ups, Geofence};

#[test]
fn geofence_rejects_latitude_outside_valid_range() {
    let ring = [[100.0, 0.0], [100.0, 1.0], [120.0, 1.0]];
    assert!(Geofence::new(&ring, 0.0, 1.0).is_err());
}

#[test]
fn non_finite_position_is_never_inside_a_polar_fence() {
    let cap = [[80.0, 0.0], [80.0, 90.0], [80.0, 180.0], [80.0, -90.0]];
    let fence = Geofence::new(&cap, 0.0, 1000.0).unwrap();
    assert!(!fence.is_point_inside_polygon(f64::NAN, f64::NAN));
}

#[test]
fn ups_rejects_latitude_above_ninety() {
    assert!(wgs84_to_ups(95.0, 0.0).is_err());
}
