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

#[test]
fn test_corners_and_center() {
    let bbox = GeoBbox::new(10.0, 20.0, 20.0, 30.0);
    assert_eq!(bbox.center(), [15.0, 25.0]);
    let corners = bbox.corners();
    assert_eq!(corners[0], [10.0, 20.0]);
    assert_eq!(corners[1], [10.0, 30.0]);
    assert_eq!(corners[2], [20.0, 30.0]);
    assert_eq!(corners[3], [20.0, 20.0]);
}

#[test]
fn test_intersects_and_expand() {
    let mut b1 = GeoBbox::new(10.0, 20.0, 15.0, 25.0);
    let b2 = GeoBbox::new(14.0, 24.0, 20.0, 30.0);
    let b3 = GeoBbox::new(30.0, 40.0, 35.0, 45.0);
    assert!(b1.intersects(&b2));
    assert!(!b1.intersects(&b3));

    b1.expand(16.0, 26.0);
    assert_eq!(b1.max_lat, 16.0);
    assert_eq!(b1.max_lon, 26.0);
    assert!(b1.contains(16.0, 26.0));
}
