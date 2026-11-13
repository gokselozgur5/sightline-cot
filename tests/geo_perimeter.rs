use sightline_cot::geo_perimeter::{
    polygon_perimeter, polygon_segment_bearings, polyline_perimeter, segment_bearing_deg,
    segment_distance_m,
};

#[test]
fn test_segment_distance_and_perimeter() {
    let p0 = [0.0, 0.0];
    let p1 = [0.0, 1.0];
    let d = segment_distance_m(p0, p1);
    let expected = 6_371_000.0 * std::f64::consts::PI / 180.0;
    assert!((d - expected).abs() < 1.0);
    let poly = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
    let perim = polygon_perimeter(&poly).unwrap();
    let sum: f64 = (0..4)
        .map(|i| segment_distance_m(poly[i], poly[(i + 1) % 4]))
        .sum();
    assert!((perim - sum).abs() < 1e-6);
}

#[test]
fn test_segment_bearings_and_degenerate() {
    let b_east = segment_bearing_deg([0.0, 0.0], [0.0, 1.0]);
    assert!((b_east - 90.0).abs() < 1e-6);
    let poly = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let bears = polygon_segment_bearings(&poly).unwrap();
    assert_eq!(bears.len(), 3);
    assert!(bears.iter().all(|&b| (0.0..360.0).contains(&b)));
    assert!(polyline_perimeter(&[[0.0, 0.0]]).is_none());
    assert!(polygon_perimeter(&[[0.0, 0.0], [1.0, 1.0]]).is_none());
    assert!(polygon_perimeter(&[[f64::NAN, 0.0], [1.0, 1.0], [0.0, 1.0]]).is_none());
}
