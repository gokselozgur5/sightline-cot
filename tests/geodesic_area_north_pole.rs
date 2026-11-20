use sightline_cot::geodesic_polygon_area;

#[test]
fn polygon_enclosing_north_pole_matches_its_southern_mirror() {
    let north = [[80.0, 0.0], [80.0, 90.0], [80.0, 180.0], [80.0, -90.0]];
    let south = [[-80.0, 0.0], [-80.0, 90.0], [-80.0, 180.0], [-80.0, -90.0]];
    let a_n = geodesic_polygon_area(&north).unwrap();
    let a_s = geodesic_polygon_area(&south).unwrap();
    assert!(a_s < 3.91e12, "south {a_s}");
    assert!(a_n < 3.91e12, "north {a_n}");
    assert!((a_n - a_s).abs() / a_s < 1e-9, "north {a_n} south {a_s}");
}
