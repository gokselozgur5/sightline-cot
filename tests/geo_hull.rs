use sightline_cot::geo_hull::convex_hull;

#[test]
fn test_interior_points_excluded() {
    let pts = [
        [37.0, -122.0],
        [37.0, -121.0],
        [38.0, -121.0],
        [38.0, -122.0],
        [37.5, -121.5],
    ];
    let hull = convex_hull(&pts);
    assert_eq!(hull.len(), 4);
    assert!(!hull.contains(&[37.5, -121.5]));
}

#[test]
fn test_collinear_returns_two_points() {
    let pts = [[37.0, -122.0], [37.5, -121.5], [38.0, -121.0]];
    let hull = convex_hull(&pts);
    assert_eq!(hull.len(), 2);
    assert_eq!(hull[0], [37.0, -122.0]);
    assert_eq!(hull[1], [38.0, -121.0]);
}
