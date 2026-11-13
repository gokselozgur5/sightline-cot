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
