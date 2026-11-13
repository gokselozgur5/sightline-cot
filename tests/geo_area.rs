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

#[test]
fn test_polygon_area_rectangle_and_winding() {
    let lat0: f64 = 45.0;
    let m = 111_194.926_6;
    let cos_lat = lat0.to_radians().cos();
    let (dlat, dlon) = (500.0 / m, 1000.0 / (cos_lat * m));
    let ccw = [
        [lat0, 10.0],
        [lat0, 10.0 + dlon],
        [lat0 + dlat, 10.0 + dlon],
        [lat0 + dlat, 10.0],
    ];
    let mut cw = ccw;
    cw.reverse();
    let (a1, a2) = (polygon_area(&ccw).unwrap(), polygon_area(&cw).unwrap());
    assert!((a1 - 500_000.0).abs() / 500_000.0 < 1e-4);
    assert!((a1 - a2).abs() < 1e-9);
    assert!((polygon_perimeter(&ccw).unwrap() - 3000.0).abs() / 3000.0 < 1e-4);
}
