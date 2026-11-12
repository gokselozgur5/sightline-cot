use sightline_cot::Geofence;

#[test]
fn band_wider_than_180_degrees_is_not_inverted() {
    let band = [
        [0.0, -100.0],
        [0.0, 0.0],
        [0.0, 100.0],
        [10.0, 100.0],
        [10.0, 0.0],
        [10.0, -100.0],
    ];
    let gf = Geofence::new(&band, 0.0, 100.0).unwrap();
    assert!(gf.is_point_inside_polygon(5.0, 0.0));
    assert!(gf.is_point_inside_polygon(5.0, -90.0));
    assert!(!gf.is_point_inside_polygon(5.0, 150.0));
    assert!(!gf.is_point_inside_polygon(5.0, -150.0));
}

#[test]
fn band_of_270_degrees_across_antimeridian() {
    let mut band = Vec::new();
    for lon in [0.0, 90.0, 180.0, -90.0] {
        band.push([0.0, lon]);
    }
    for lon in [-90.0, 180.0, 90.0, 0.0] {
        band.push([10.0, lon]);
    }
    let gf = Geofence::new(&band, 0.0, 100.0).unwrap();
    assert!(gf.is_point_inside_polygon(5.0, -135.0));
    assert!(gf.is_point_inside_polygon(5.0, 179.0));
    assert!(!gf.is_point_inside_polygon(5.0, -45.0));
}
