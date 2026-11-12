//! Known-coordinate and roundtrip tests for the MGRS and UTM converter.

use sightline_cot::{utm_to_wgs84, wgs84_to_utm};

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

#[test]
fn zone31_meridian_arc_anchors() {
    let cases = [(45.0, 4982950.0), (60.0, 6651411.2)];
    for (lat, northing) in cases {
        let utm = wgs84_to_utm(lat, 3.0).unwrap();
        assert_eq!(utm.zone, 31);
        assert_close(utm.easting, 500000.0, 1e-6, "easting");
        assert_close(utm.northing, northing, 100.0, "northing");
    }
}

#[test]
fn roundtrip_all_zones_both_hemispheres() {
    let lats = [-79.5, -45.0, -0.5, 0.5, 45.0, 83.9];
    for zone in 1..=60u8 {
        let cm = (zone as f64 - 1.0) * 6.0 - 177.0;
        for lon in [cm - 2.9, cm, cm + 2.9] {
            for lat in lats {
                let utm = wgs84_to_utm(lat, lon).unwrap();
                assert_eq!(utm.zone, zone);
                assert_eq!(utm.hemisphere, if lat < 0.0 { 'S' } else { 'N' });
                let (lat_rt, lon_rt) = utm_to_wgs84(&utm).unwrap();
                assert_close(lat_rt, lat, 1e-6, "lat roundtrip");
                assert_close(lon_rt, lon, 1e-6, "lon roundtrip");
            }
        }
    }
}
