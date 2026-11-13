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

#[test]
fn test_counter_clockwise_orientation() {
    let pts = [
        [10.0, 20.0],
        [12.0, 20.0],
        [12.0, 25.0],
        [10.0, 25.0],
        [11.0, 22.0],
    ];
    let hull = convex_hull(&pts);
    assert_eq!(hull.len(), 4);
    let mut area2 = 0.0;
    for i in 0..hull.len() {
        let (a, b) = (hull[i], hull[(i + 1) % hull.len()]);
        area2 += a[1] * b[0] - b[1] * a[0];
    }
    assert!(area2 > 0.0);
}

#[test]
fn test_small_inputs() {
    assert!(convex_hull(&[]).is_empty());
    assert_eq!(convex_hull(&[[10.0, 20.0]]), vec![[10.0, 20.0]]);
    assert_eq!(
        convex_hull(&[[1.0, 2.0], [3.0, 4.0]]),
        vec![[1.0, 2.0], [3.0, 4.0]]
    );
}
