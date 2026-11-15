use sightline_cot::geo_vincenty::{vincenty_direct, vincenty_inverse};

#[test]
fn test_flinders_peak_to_buninyong() {
    let lat1 = -(37.0 + 57.0 / 60.0 + 3.72030 / 3600.0);
    let lon1 = 144.0 + 25.0 / 60.0 + 29.52440 / 3600.0;
    let lat2 = -(37.0 + 39.0 / 60.0 + 10.15610 / 3600.0);
    let lon2 = 143.0 + 55.0 / 60.0 + 35.38390 / 3600.0;
    let inv = vincenty_inverse(lat1, lon1, lat2, lon2).expect("inverse");
    assert!(
        (inv.distance_m - 54972.271).abs() < 0.001,
        "distance diff {} >= 1mm",
        (inv.distance_m - 54972.271).abs()
    );
}

#[test]
fn test_direct_recovers_inverse() {
    let lat1 = -(37.0 + 57.0 / 60.0 + 3.72030 / 3600.0);
    let lon1 = 144.0 + 25.0 / 60.0 + 29.52440 / 3600.0;
    let lat2 = -(37.0 + 39.0 / 60.0 + 10.15610 / 3600.0);
    let lon2 = 143.0 + 55.0 / 60.0 + 35.38390 / 3600.0;
    let inv = vincenty_inverse(lat1, lon1, lat2, lon2).expect("inverse");
    let dir = vincenty_direct(lat1, lon1, inv.forward_azimuth_deg, inv.distance_m).expect("direct");
    assert!((dir.lat_deg - lat2).abs() < 1e-7);
    assert!((dir.lon_deg - lon2).abs() < 1e-7);
    assert!((dir.reverse_azimuth_deg - inv.reverse_azimuth_deg).abs() < 1e-5);
}

use sightline_cot::geo_vincenty::VincentyError;

#[test]
fn test_antipodal_and_error_cases() {
    assert_eq!(
        vincenty_inverse(0.0, 0.0, 0.5, 179.7),
        Err(VincentyError::NoConvergence)
    );
    assert_eq!(
        vincenty_inverse(0.0, 0.0, 0.0, 180.0),
        Err(VincentyError::NoConvergence)
    );

    assert_eq!(
        vincenty_inverse(95.0, 0.0, 0.0, 0.0),
        Err(VincentyError::InvalidLatitude)
    );
    assert_eq!(
        vincenty_direct(0.0, 0.0, 0.0, -10.0),
        Err(VincentyError::NegativeDistance)
    );
    assert_eq!(
        vincenty_direct(0.0, 0.0, f64::NAN, 100.0),
        Err(VincentyError::NonFiniteInput)
    );
}

#[test]
fn test_polar_coincident_points() {
    let north = vincenty_inverse(90.0, 0.0, 90.0, 45.0).expect("north pole");
    assert_eq!(north.distance_m, 0.0);
    assert_eq!(north.forward_azimuth_deg, 0.0);
    assert_eq!(north.reverse_azimuth_deg, 0.0);

    let south = vincenty_inverse(-90.0, 10.0, -90.0, -50.0).expect("south pole");
    assert_eq!(south.distance_m, 0.0);
}

#[test]
fn test_equator_and_meridian_arcs() {
    let eq = vincenty_inverse(0.0, 0.0, 0.0, 1.0).expect("equator 1 deg");
    assert!((eq.distance_m - 111319.491).abs() < 0.01);
    assert!((eq.forward_azimuth_deg - 90.0).abs() < 1e-6);

    let mer = vincenty_inverse(0.0, 0.0, 90.0, 0.0).expect("quarter meridian");
    assert!((mer.distance_m - 10001965.729).abs() < 0.01);
    assert!((mer.forward_azimuth_deg - 0.0).abs() < 1e-6);
}
