use sightline_cot::geo_area::{
    planar_polygon_area, planar_polygon_perimeter, polygon_area, polygon_perimeter,
};

#[test]
fn test_planar_area_and_perimeter_square() {
    let sq = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
    let a = planar_polygon_area(&sq).unwrap();
    let p = planar_polygon_perimeter(&sq).unwrap();
    assert!((a - 100.0).abs() < 1e-12);
    assert!((p - 40.0).abs() < 1e-12);
}

#[test]
fn test_degenerate_polygons() {
    assert!(polygon_area(&[[0.0, 0.0], [1.0, 1.0]]).is_none());
    assert!(polygon_perimeter(&[[0.0, 0.0], [1.0, 1.0]]).is_none());
    assert!(polygon_area(&[[f64::NAN, 0.0], [1.0, 1.0], [0.0, 1.0]]).is_none());
}
