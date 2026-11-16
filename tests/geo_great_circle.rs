use sightline_cot::geo_great_circle::{GreatCirclePath, Waypoint};

#[test]
fn test_integration_transcontinental_flight_path() {
    let jfk = Waypoint::new(40.6413, -73.7781);
    let lhr = Waypoint::new(51.4700, -0.4543);
    let hnd = Waypoint::new(35.5494, 139.7798);

    let flight_plan = [jfk, lhr, hnd];
    let path = GreatCirclePath::from_waypoints(&flight_plan).expect("build path");

    assert_eq!(path.waypoint_count(), 3);
    assert_eq!(path.segment_count(), 2);
    assert!(path.total_distance_m() > 14_000_000.0);

    let start = path.interpolate_at_fraction(0.0).unwrap();
    assert!((start.lat - jfk.lat).abs() < 1e-6);

    let mid = path.interpolate_at_fraction(0.5).unwrap();
    assert!(mid.lat.is_finite() && mid.lon.is_finite());

    let end = path.interpolate_at_fraction(1.0).unwrap();
    assert!((end.lat - hnd.lat).abs() < 1e-6);
}
