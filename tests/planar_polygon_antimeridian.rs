use sightline_cot::geo_area::{polygon_area, polygon_perimeter};
use sightline_cot::geo_centroid::polygon_centroid;

const ACROSS: [[f64; 2]; 4] = [[10.0, 179.0], [10.0, -179.0], [11.0, -179.0], [11.0, 179.0]];
const SHIFTED: [[f64; 2]; 4] = [[10.0, 9.0], [10.0, 11.0], [11.0, 11.0], [11.0, 9.0]];

#[test]
fn area_and_perimeter_do_not_depend_on_the_antimeridian() {
    let (a, a_ref) = (polygon_area(&ACROSS).unwrap(), polygon_area(&SHIFTED).unwrap());
    assert!((a_ref - 2.4314e10).abs() / 2.4314e10 < 1e-3);
    assert!((a - a_ref).abs() / a_ref < 1e-6, "area {a} expected {a_ref}");
    let (p, p_ref) = (polygon_perimeter(&ACROSS).unwrap(), polygon_perimeter(&SHIFTED).unwrap());
    assert!((p - p_ref).abs() / p_ref < 1e-6, "perimeter {p} expected {p_ref}");
}

#[test]
fn centroid_of_antimeridian_polygon_lies_on_the_antimeridian() {
    let c = polygon_centroid(&ACROSS).unwrap();
    assert!((c[0] - 10.5).abs() < 1e-6);
    assert!(c[1].abs() > 179.999, "centroid longitude {}", c[1]);
}
