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
