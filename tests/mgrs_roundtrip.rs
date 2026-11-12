//! Known-coordinate and roundtrip tests for the MGRS and UTM converter.

use sightline_cot::wgs84_to_utm;

fn assert_close(actual: f64, expected: f64, tol: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{label}: expected {expected}, got {actual} (tolerance {tol})"
    );
}

#[test]
fn central_meridians_project_to_false_origin() {
    for zone in 1..=60u8 {
        let lon = (zone as f64 - 1.0) * 6.0 - 177.0;
        let utm = wgs84_to_utm(0.0, lon).unwrap();
        assert_eq!(utm.zone, zone);
        assert_eq!(utm.hemisphere, 'N');
        assert_close(utm.easting, 500000.0, 1e-6, "easting");
        assert_close(utm.northing, 0.0, 1e-6, "northing");
    }
}
