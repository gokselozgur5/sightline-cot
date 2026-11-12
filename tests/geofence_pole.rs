use sightline_cot::Geofence;

#[test]
fn ring_around_north_pole_encloses_the_pole_cap() {
    let ring = [[80.0, 0.0], [80.0, 90.0], [80.0, 180.0], [80.0, -90.0]];
    let gf = Geofence::new(&ring, 0.0, 100.0).unwrap();
    assert!(gf.is_point_inside_polygon(85.0, 45.0));
    assert!(gf.is_point_inside_polygon(89.9, -170.0));
    assert!(!gf.is_point_inside_polygon(70.0, 45.0));
    assert!(!gf.is_point_inside_polygon(0.0, 0.0));
}

#[test]
fn ring_around_south_pole_encloses_the_pole_cap() {
    let ring = [[-75.0, 170.0], [-80.0, -100.0], [-78.0, 0.0], [-80.0, 80.0]];
    let gf = Geofence::new(&ring, 0.0, 100.0).unwrap();
    assert!(gf.is_point_inside_polygon(-85.0, 45.0));
    assert!(gf.is_point_inside_polygon(-89.9, 179.9));
    assert!(!gf.is_point_inside_polygon(-60.0, -100.0));
    assert!(!gf.is_point_inside_polygon(10.0, 0.0));
}
