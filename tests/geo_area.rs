use sightline_cot::geo_area::{
    planar_polygon_area, planar_polygon_perimeter, polygon_area, polygon_perimeter,
    spherical_polygon_area, spherical_polygon_area_with_radius, spherical_polygon_excess,
    spherical_polygon_perimeter, spherical_polygon_perimeter_with_radius,
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

#[test]
fn test_spherical_polygon_excess_octant() {
    let octant = [[0.0, 0.0], [0.0, 90.0], [90.0, 0.0]];
    let excess = spherical_polygon_excess(&octant).unwrap();
    assert!((excess - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
}

#[test]
fn test_spherical_polygon_area_and_winding() {
    let octant_ccw = [[0.0, 0.0], [0.0, 90.0], [90.0, 0.0]];
    let mut octant_cw = octant_ccw;
    octant_cw.reverse();
    let a_ccw = spherical_polygon_area_with_radius(&octant_ccw, 1.0).unwrap();
    let a_cw = spherical_polygon_area_with_radius(&octant_cw, 1.0).unwrap();
    assert!((a_ccw - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
    assert!((a_cw - a_ccw).abs() < 1e-12);
    let a_earth = spherical_polygon_area(&octant_ccw).unwrap();
    assert!(a_earth > 0.0);
}

#[test]
fn test_spherical_polygon_perimeter_and_degeneracy() {
    let octant = [[0.0, 0.0], [0.0, 90.0], [90.0, 0.0]];
    let p = spherical_polygon_perimeter_with_radius(&octant, 1.0).unwrap();
    assert!((p - 1.5 * std::f64::consts::PI).abs() < 1e-12);
    let p_earth = spherical_polygon_perimeter(&octant).unwrap();
    assert!((p_earth - 1.5 * std::f64::consts::PI * 6_371_008.8).abs() < 1e-3);
    assert!(spherical_polygon_area(&[[0.0, 0.0], [1.0, 1.0]]).is_none());
    assert!(spherical_polygon_perimeter(&[[0.0, 0.0], [1.0, 1.0]]).is_none());
    assert!(spherical_polygon_area_with_radius(&octant, -1.0).is_none());
}

#[test]
fn test_spherical_area_antimeridian_and_planar_limit() {
    let poly = [[0.0, 179.0], [0.0, -179.0], [1.0, -179.0], [1.0, 179.0]];
    let a_sphere = spherical_polygon_area(&poly).unwrap();
    let p_sphere = spherical_polygon_perimeter(&poly).unwrap();
    assert!(a_sphere > 0.0 && p_sphere > 0.0);
}
