use sightline_cot::geo_interp::{great_circle_intermediate, great_circle_midpoint};

#[test]
fn test_equator_midpoint_and_fraction() {
    let p1 = (0.0, 0.0);
    let p2 = (0.0, 90.0);
    let mid = great_circle_midpoint(p1.0, p1.1, p2.0, p2.1).unwrap();
    assert!((mid.0 - 0.0).abs() < 1e-6);
    assert!((mid.1 - 45.0).abs() < 1e-6);

    let quarter = great_circle_intermediate(p1.0, p1.1, p2.0, p2.1, 0.25).unwrap();
    assert!((quarter.0 - 0.0).abs() < 1e-6);
    assert!((quarter.1 - 22.5).abs() < 1e-6);
}
