use sightline_cot::{simplify_polyline, LocalFrame};

#[test]
fn test_endpoints_kept_and_collinear_removed() {
    let pts = vec![
        [45.0, 10.0],
        [45.002, 10.0],
        [45.004, 10.0],
        [45.006, 10.0],
        [45.010, 10.0],
    ];
    let res = simplify_polyline(&pts, 1.0);
    assert_eq!(res.len(), 2);
    assert_eq!(res[0], [45.0, 10.0]);
    assert_eq!(res[1], [45.010, 10.0]);
}
