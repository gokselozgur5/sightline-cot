use sightline_cot::{has_line_of_sight, GeoPoint3D};

#[test]
fn nearby_points_with_negative_hae_see_each_other() {
    let a = GeoPoint3D::new(8.0, 77.5, -60.0).unwrap();
    let b = GeoPoint3D::new(8.0, 77.501, -60.0).unwrap();
    assert!(has_line_of_sight(&a, &b).unwrap());
}
