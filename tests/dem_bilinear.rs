use sightline_cot::dem_bilinear::{sample_dem_bilinear, DemError, DemRaster};

#[test]
fn test_exact_at_nodes() {
    let mut elev = Vec::with_capacity(9);
    for i in 1..=9 {
        elev.push(Some(i as f64 * 10.0));
    }
    let dem = DemRaster::new(100.0, 200.0, 10.0, 3, 3, elev).unwrap();
    for r in 0..3 {
        for c in 0..3 {
            let (e, n) = (100.0 + c as f64 * 10.0, 200.0 + r as f64 * 10.0);
            let expected = (r * 3 + c + 1) as f64 * 10.0;
            assert_eq!(dem.sample(e, n), Some(expected));
            assert_eq!(sample_dem_bilinear(&dem, e, n), Some(expected));
        }
    }
}
