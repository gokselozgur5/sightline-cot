use sightline_cot::geo_bbox::GeoBbox;

#[test]
fn test_from_points_tightly_encloses() {
    let pts = [[10.0, 20.0], [15.0, 25.0], [12.0, 18.0]];
    let bbox = GeoBbox::from_points(&pts).unwrap();
    assert_eq!(bbox.min_lat, 10.0);
    assert_eq!(bbox.max_lat, 15.0);
    assert_eq!(bbox.min_lon, 18.0);
    assert_eq!(bbox.max_lon, 25.0);
    assert!(bbox.contains(12.0, 20.0));
    assert!(!bbox.contains(9.0, 20.0));
}

#[test]
fn test_empty_input_returns_none() {
    assert!(GeoBbox::from_points(&[]).is_none());
}
