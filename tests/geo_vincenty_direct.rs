use sightline_cot::geo_vincenty_direct::vincenty_direct;

#[test]
fn test_vincenty_direct_flinders_peak() {
    let lat1 = -(37.0 + 57.0 / 60.0 + 3.72030 / 3600.0);
    let lon1 = 144.0 + 25.0 / 60.0 + 29.52440 / 3600.0;
    let alpha1 = 306.0 + 52.0 / 60.0 + 5.37 / 3600.0;
    let s = 54972.271;
    let expected_lat = -(37.0 + 39.0 / 60.0 + 10.15610 / 3600.0);
    let expected_lon = 143.0 + 55.0 / 60.0 + 35.38390 / 3600.0;

    let res = vincenty_direct(lat1, lon1, alpha1, s).expect("direct solve");
    assert!((res.lat_deg - expected_lat).abs() < 1e-6);
    assert!((res.lon_deg - expected_lon).abs() < 1e-6);
}

use sightline_cot::geo_vincenty_direct::VincentyError;

#[test]
fn test_vincenty_direct_validation_errors() {
    assert_eq!(vincenty_direct(95.0, 0.0, 0.0, 100.0), Err(VincentyError::InvalidLatitude));
    assert_eq!(vincenty_direct(0.0, 185.0, 0.0, 100.0), Err(VincentyError::InvalidLongitude));
    assert_eq!(vincenty_direct(0.0, 0.0, 0.0, -10.0), Err(VincentyError::NegativeDistance));
    assert_eq!(vincenty_direct(0.0, 0.0, f64::NAN, 10.0), Err(VincentyError::NonFiniteInput));
}

#[test]
fn test_vincenty_direct_zero_distance() {
    let res = vincenty_direct(10.0, 20.0, 45.0, 0.0).expect("zero dist");
    assert!((res.lat_deg - 10.0).abs() < 1e-9);
    assert!((res.lon_deg - 20.0).abs() < 1e-9);
}
