use sightline_cot::geo_dest::{destination_point, haversine_distance, initial_bearing};

#[test]
fn test_destination_acceptance_roundtrip_range() {
    let lat = 37.7749;
    let lon = -122.4194;
    let bearing = 60.0;
    let range = 25_000.0;
    let (dest_lat, dest_lon) = destination_point(lat, lon, bearing, range).expect("destination");
    let back_dist = haversine_distance(lat, lon, dest_lat, dest_lon);
    let rel_err = (back_dist - range).abs() / range;
    assert!(rel_err < 1e-4, "relative error {} >= 1e-4", rel_err);
    let init_b = initial_bearing(lat, lon, dest_lat, dest_lon);
    assert!((init_b - bearing).abs() < 1e-4);
}

#[test]
fn test_destination_antimeridian_and_polar() {
    let (dest_lat, dest_lon) = destination_point(0.0, 179.95, 90.0, 20_000.0).expect("cross east");
    assert!(dest_lon < 0.0);
    let d = haversine_distance(0.0, 179.95, dest_lat, dest_lon);
    assert!((d - 20_000.0).abs() < 1.0);

    let (pole_lat, _) = destination_point(89.5, 0.0, 0.0, 50_000.0).expect("polar dest");
    assert!((-90.0..=90.0).contains(&pole_lat));

    let (zero_lat, zero_lon) = destination_point(45.0, 10.0, 120.0, 0.0).expect("zero range");
    assert!((zero_lat - 45.0).abs() < 1e-9);
    assert!((zero_lon - 10.0).abs() < 1e-9);
}

use sightline_cot::geo_dest::GeoDestError;

#[test]
fn test_destination_error_cases() {
    assert_eq!(
        destination_point(95.0, 0.0, 0.0, 1000.0),
        Err(GeoDestError::InvalidLatitude)
    );
    assert_eq!(
        destination_point(0.0, 190.0, 0.0, 1000.0),
        Err(GeoDestError::InvalidLongitude)
    );
    assert_eq!(
        destination_point(0.0, 0.0, 0.0, -100.0),
        Err(GeoDestError::NegativeRange)
    );
    assert_eq!(
        destination_point(f64::NAN, 0.0, 0.0, 100.0),
        Err(GeoDestError::NonFiniteInput)
    );
}
