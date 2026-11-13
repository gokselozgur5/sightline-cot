use sightline_cot::geo_centroid::{planar_centroid, polygon_centroid};

#[test]
fn test_planar_centroid_square() {
    let sq = [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]];
    let c = planar_centroid(&sq).unwrap();
    assert!((c[0] - 1.0).abs() < 1e-12 && (c[1] - 1.0).abs() < 1e-12);
}

#[test]
fn test_symmetric_rectangle_and_winding() {
    let ccw = [[44.99, 9.99], [44.99, 10.01], [45.01, 10.01], [45.01, 9.99]];
    let mut cw = ccw;
    cw.reverse();
    let c1 = polygon_centroid(&ccw).unwrap();
    let c2 = polygon_centroid(&cw).unwrap();
    assert!((c1[0] - 45.0).abs() < 1e-9 && (c1[1] - 10.0).abs() < 1e-9);
    assert!((c1[0] - c2[0]).abs() < 1e-12 && (c1[1] - c2[1]).abs() < 1e-12);
}

#[test]
fn test_degenerate_polygons() {
    assert!(polygon_centroid(&[[0.0, 0.0], [1.0, 1.0]]).is_none());
    assert!(polygon_centroid(&[[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]]).is_none());
    assert!(polygon_centroid(&[[f64::NAN, 0.0], [1.0, 1.0], [0.0, 1.0]]).is_none());
}
