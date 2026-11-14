use sightline_cot::{along_track_distance_m, cross_track_distance_m, cross_track_error};

#[test]
fn test_zero_crosstrack_on_path() {
    let start = [0.0, 0.0];
    let end = [0.0, 10.0];
    let on_path = [0.0, 5.0];
    let d = cross_track_distance_m(start, end, on_path);
    assert!(d.abs() < 1e-6);
}

#[test]
fn test_sign_flips_across_path() {
    let start = [0.0, 0.0];
    let end = [0.0, 10.0];
    let left = [1.0, 5.0];
    let right = [-1.0, 5.0];
    let d_left = cross_track_distance_m(start, end, left);
    let d_right = cross_track_distance_m(start, end, right);
    assert!(d_left < 0.0);
    assert!(d_right > 0.0);
    assert!((d_left + d_right).abs() < 1e-4);
}

#[test]
fn test_along_track_and_combined_error() {
    let start = [0.0, 0.0];
    let end = [0.0, 10.0];
    let pt = [1.0, 5.0];
    let res = cross_track_error(start, end, pt);
    let at = along_track_distance_m(start, end, pt);
    assert_eq!(res.cross_track_m, cross_track_distance_m(start, end, pt));
    assert_eq!(res.along_track_m, at);
    assert!(at > 0.0);
}
