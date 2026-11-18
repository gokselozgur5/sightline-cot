use sightline_cot::geo_hull::convex_hull;

#[test]
fn hull_output_has_no_nan_and_no_repeated_vertex() {
    let pts = [[0.0, 0.0], [f64::NAN, 1.0], [1.0, 1.0], [1.0, 0.0], [0.5, 0.5]];
    let hull = convex_hull(&pts);
    assert!(hull.iter().flatten().all(|c| c.is_finite()), "hull {hull:?}");
    for (i, p) in hull.iter().enumerate() {
        assert!(!hull[i + 1..].contains(p), "repeated vertex in {hull:?}");
    }
}

#[test]
fn hull_with_small_input_and_nan() {
    let pts = [[0.0, 0.0], [f64::NAN, 1.0]];
    let hull = convex_hull(&pts);
    assert!(hull.iter().flatten().all(|c| c.is_finite()), "hull {hull:?}");
}
