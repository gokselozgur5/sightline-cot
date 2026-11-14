use sightline_cot::geo_interp::{great_circle_intermediate, great_circle_midpoint, InterpError};

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

#[test]
fn test_interp_error_cases() {
    assert_eq!(
        great_circle_intermediate(95.0, 0.0, 0.0, 0.0, 0.5),
        Err(InterpError::InvalidLatitude)
    );
    assert_eq!(
        great_circle_intermediate(0.0, 200.0, 0.0, 0.0, 0.5),
        Err(InterpError::InvalidLongitude)
    );
    assert_eq!(
        great_circle_intermediate(0.0, 0.0, 0.0, 0.0, 1.5),
        Err(InterpError::InvalidFraction)
    );
    assert_eq!(
        great_circle_intermediate(0.0, 0.0, 0.0, 180.0, 0.5),
        Err(InterpError::AntipodalPoints)
    );
}

#[test]
fn test_antimeridian_and_polar_midpoint() {
    let p1 = (10.0, 179.0);
    let p2 = (10.0, -179.0);
    let mid = great_circle_midpoint(p1.0, p1.1, p2.0, p2.1).unwrap();
    assert!((mid.0 - 10.0).abs() < 0.01);
    assert!((mid.1.abs() - 180.0).abs() < 1e-4);

    let pole_mid = great_circle_midpoint(80.0, 0.0, 80.0, 180.0).unwrap();
    assert!((pole_mid.0 - 90.0).abs() < 1e-4);
}

#[test]
fn test_identical_endpoints_and_boundary_fractions() {
    let p = (37.7749, -122.4194);
    let mid_same = great_circle_midpoint(p.0, p.1, p.0, p.1).unwrap();
    assert!((mid_same.0 - p.0).abs() < 1e-6);
    assert!((mid_same.1 - p.1).abs() < 1e-6);

    let p2 = (34.0522, -118.2437);
    let start = great_circle_intermediate(p.0, p.1, p2.0, p2.1, 0.0).unwrap();
    assert!((start.0 - p.0).abs() < 1e-6);
    assert!((start.1 - p.1).abs() < 1e-6);

    let end = great_circle_intermediate(p.0, p.1, p2.0, p2.1, 1.0).unwrap();
    assert!((end.0 - p2.0).abs() < 1e-6);
    assert!((end.1 - p2.1).abs() < 1e-6);
}
