use sightline_cot::{GreatCirclePath, Waypoint};

#[test]
fn bearing_at_final_waypoint_is_the_arrival_course() {
    let path =
        GreatCirclePath::from_waypoints(&[Waypoint::new(0.0, 0.0), Waypoint::new(0.0, 10.0)])
            .unwrap();
    let total = path.total_distance_m();
    let near_end = path.bearing_at_distance(total - 1.0).unwrap();
    let at_end = path.bearing_at_distance(total).unwrap();
    assert!((near_end - 90.0).abs() < 1e-6);
    assert!((at_end - 90.0).abs() < 1e-6, "bearing at end {at_end}");
}
