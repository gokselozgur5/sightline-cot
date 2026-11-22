// One triangle, three public perimeter functions: all sum great-circle edge lengths,
// so they must agree to rounding on the same vertices.
use sightline_cot::geo_area::spherical_polygon_perimeter;
use sightline_cot::geo_perimeter::polygon_perimeter;
use sightline_cot::polygon_geodesic_perimeter;

#[test]
fn perimeter_functions_agree_on_the_same_triangle() {
    let tri = [[60.0, 0.0], [60.0, 20.0], [70.0, 10.0]];
    let a = polygon_perimeter(&tri).unwrap();
    let b = spherical_polygon_perimeter(&tri).unwrap();
    let c = polygon_geodesic_perimeter(&tri).unwrap();
    assert!((a - b).abs() < 1e-3, "geo_perimeter {a} vs geo_area {b}");
    assert!((b - c).abs() < 1e-3, "geo_area {b} vs geo_polygon_area {c}");
}
