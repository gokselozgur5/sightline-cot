use sightline_cot::geoid::{ConstantGeoid, GeoidModel};

#[test]
fn test_constant_geoid_separation() {
    let geoid = ConstantGeoid::new(30.0);
    assert_eq!(geoid.undulation(45.0, 10.0), Some(30.0));
    assert_eq!(
        geoid.orthometric_to_ellipsoidal(45.0, 10.0, 100.0),
        Some(130.0)
    );
    assert_eq!(
        geoid.ellipsoidal_to_orthometric(45.0, 10.0, 130.0),
        Some(100.0)
    );
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
    assert!((grid.undulation(0.5, 0.0).unwrap() - 20.0).abs() < 1e-9);
    assert!((grid.undulation(0.0, 0.5).unwrap() - 15.0).abs() < 1e-9);
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
    let ellipsoidal_h = grid
        .orthometric_to_ellipsoidal(lat, lon, orthometric_h)
        .unwrap();
    let recovered_h = grid
        .ellipsoidal_to_orthometric(lat, lon, ellipsoidal_h)
        .unwrap();
    assert!((recovered_h - orthometric_h).abs() < 1e-9);
}

#[test]
fn test_from_f64_sentinel() {
    let raw = [20.0, -9999.0, f64::NAN, 50.0];
    let grid = GeoidGrid::from_f64_uniform(0.0, 0.0, 1.0, 2, 2, &raw, Some(-9999.0)).unwrap();
    assert_eq!(grid.get_node(1, 0), None);
    assert_eq!(grid.get_node(0, 1), None);
    assert_eq!(grid.get_node(0, 0), Some(20.0));
    assert_eq!(grid.get_node(1, 1), Some(50.0));
    assert_eq!(grid.undulation(0.5, 0.5), None);
}

use sightline_cot::geoid::GeoidError;

#[test]
fn test_grid_validation_errors() {
    let u = vec![None; 4];
    assert_eq!(
        GeoidGrid::new_uniform(0.0, 0.0, 0.0, 2, 2, u.clone()),
        Err(GeoidError::InvalidStep)
    );
    assert_eq!(
        GeoidGrid::new_uniform(0.0, 0.0, 1.0, 1, 2, u.clone()),
        Err(GeoidError::InvalidDimensions)
    );
    assert_eq!(
        GeoidGrid::new_uniform(0.0, 0.0, 1.0, 2, 2, vec![None; 3]),
        Err(GeoidError::DataLengthMismatch)
    );
    assert_eq!(
        GeoidGrid::new_uniform(89.5, 0.0, 1.0, 2, 2, u.clone()),
        Err(GeoidError::InvalidBounds)
    );
    assert_eq!(
        GeoidGrid::new_uniform(f64::NAN, 0.0, 1.0, 2, 2, u),
        Err(GeoidError::NonFiniteCoordinate)
    );
}

#[test]
fn test_trait_dispatch() {
    let geoid = ConstantGeoid::new(12.5);
    let trait_ref: &dyn GeoidModel = &geoid;
    let boxed: Box<dyn GeoidModel> = Box::new(geoid);
    assert_eq!(trait_ref.undulation(10.0, 10.0), Some(12.5));
    assert_eq!(boxed.undulation(10.0, 10.0), Some(12.5));
    assert_eq!(
        sightline_cot::geoid::orthometric_to_ellipsoidal(&geoid, 10.0, 10.0, 50.0),
        Some(62.5)
    );
    assert_eq!(
        sightline_cot::geoid::ellipsoidal_to_orthometric(&boxed, 10.0, 10.0, 62.5),
        Some(50.0)
    );
}

#[test]
fn test_geoid_loader_ascii_grid_interop() {
    let ascii = "ncols 2\nnrows 2\nxllcorner 0.0\nyllcorner 0.0\ncellsize 1.0\n10.0 20.0\n30.0 40.0\n";
    let grid = sightline_cot::geoid_loader::parse_geoid_ascii(ascii).unwrap();
    let ortho = sightline_cot::ellipsoidal_to_orthometric(&grid, 1.0, 1.0, 125.0);
    assert_eq!(ortho, Some(100.0));
    let ellips = sightline_cot::orthometric_to_ellipsoidal(&grid, 1.0, 1.0, 100.0);
    assert_eq!(ellips, Some(125.0));
}
