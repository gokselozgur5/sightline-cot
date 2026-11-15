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

#[test]
fn test_bilinear_planar_surface() {
    let vals = vec![Some(10.0), Some(20.0), Some(30.0), Some(40.0)];
    let grid = GeoidGrid::new_uniform(0.0, 0.0, 1.0, 2, 2, vals).unwrap();
    assert!((grid.undulation(0.5, 0.0).unwrap() - 15.0).abs() < 1e-9);
    assert!((grid.undulation(0.0, 0.5).unwrap() - 20.0).abs() < 1e-9);
    assert!((grid.undulation(0.5, 0.5).unwrap() - 25.0).abs() < 1e-9);
    let (lat, lon) = (0.25, 0.75);
    let expected = 10.0 + 20.0 * lat + 10.0 * lon;
    assert!((grid.undulation(lat, lon).unwrap() - expected).abs() < 1e-9);
}

#[test]
fn test_nodata_corner_rejection() {
    let mut vals = vec![Some(15.0); 9];
    vals[4] = None;
    let grid = GeoidGrid::new_uniform(10.0, 20.0, 1.0, 3, 3, vals).unwrap();
    assert_eq!(grid.undulation(10.5, 20.5), None);
    assert_eq!(grid.undulation(11.5, 20.5), None);
    assert_eq!(grid.undulation(10.5, 21.5), None);
    assert_eq!(grid.undulation(11.5, 21.5), None);
    assert_eq!(grid.undulation(9.0, 20.0), None);
}

#[test]
fn test_orthometric_ellipsoidal_roundtrip() {
    let vals = vec![Some(25.0), Some(27.0), Some(29.0), Some(31.0)];
    let grid = GeoidGrid::new_uniform(35.0, 45.0, 0.5, 2, 2, vals).unwrap();
    let (lat, lon) = (35.2, 45.3);
    let orthometric_h = 1250.5;
    let ellipsoidal_h = grid.orthometric_to_ellipsoidal(lat, lon, orthometric_h).unwrap();
    let recovered_h = grid.ellipsoidal_to_orthometric(lat, lon, ellipsoidal_h).unwrap();
    assert!((recovered_h - orthometric_h).abs() < 1e-9);
}
