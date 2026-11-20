use sightline_cot::geo_area::spherical_polygon_area;
use sightline_cot::geodesic_polygon_area;

#[test]
fn geodesic_area_of_large_high_latitude_triangle_is_within_two_percent() {
    let tri = [[60.0, 0.0], [60.0, 20.0], [70.0, 10.0]];
    let reference = 5.9433e11;
    let sphere = spherical_polygon_area(&tri).unwrap();
    assert!((sphere - reference).abs() / reference < 1e-4);
    let area = geodesic_polygon_area(&tri).unwrap();
    let rel = (area - reference).abs() / reference;
    assert!(rel < 0.02, "area {area} differs from {reference} by {rel}");
}
