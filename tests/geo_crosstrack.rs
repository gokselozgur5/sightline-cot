use sightline_cot::cross_track_distance_m;

#[test]
fn test_zero_crosstrack_on_path() {
    let start = [0.0, 0.0];
    let end = [0.0, 10.0];
    let on_path = [0.0, 5.0];
    let d = cross_track_distance_m(start, end, on_path);
    assert!(d.abs() < 1e-6);
}
