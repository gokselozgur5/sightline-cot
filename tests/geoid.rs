use sightline_cot::geoid::{ConstantGeoid, GeoidModel};

#[test]
fn test_constant_geoid_separation() {
    let geoid = ConstantGeoid::new(30.0);
    assert_eq!(geoid.undulation(45.0, 10.0), Some(30.0));
    assert_eq!(geoid.orthometric_to_ellipsoidal(45.0, 10.0, 100.0), Some(130.0));
    assert_eq!(geoid.ellipsoidal_to_orthometric(45.0, 10.0, 130.0), Some(100.0));
}

use sightline_cot::geoid::{sample_geoid_bilinear, GeoidGrid};

#[test]
fn test_exact_at_nodes() {
    let mut und = Vec::with_capacity(9);
    for i in 1..=9 {
        und.push(Some(i as f64 * 5.0));
    }
    let grid = GeoidGrid::new(40.0, 30.0, 0.5, 0.5, 3, 3, und).unwrap();
    for r in 0..3 {
        for c in 0..3 {
            let lat = 40.0 + r as f64 * 0.5;
            let lon = 30.0 + c as f64 * 0.5;
            let expected = (r * 3 + c + 1) as f64 * 5.0;
            assert_eq!(grid.undulation(lat, lon), Some(expected));
            assert_eq!(sample_geoid_bilinear(&grid, lat, lon), Some(expected));
        }
    }
}
