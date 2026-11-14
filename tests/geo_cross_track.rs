use sightline_cot::geo_cross_track::cross_track_distance;

#[test]
fn test_on_path_and_offsets() {
    let (s, e) = ((0.0, 0.0), (0.0, 10.0));
    assert!(cross_track_distance(s, e, (0.0, 5.0)).abs() < 1e-6);
    let right = cross_track_distance(s, e, (-1.0, 5.0));
    assert!((right - 111_195.0).abs() < 200.0);
    let left = cross_track_distance(s, e, (1.0, 5.0));
    assert!((left + 111_195.0).abs() < 200.0);
}
