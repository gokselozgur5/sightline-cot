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

#[test]
fn test_vincenty_inverse_ellipsoids() {
    use sightline_cot::ellipsoid::Ellipsoid;
    use sightline_cot::geo_vincenty::vincenty_inverse_ellipsoid;
    let lat1 = 37.7749;
    let lon1 = -122.4194;
    let lat2 = 34.0522;
    let lon2 = -118.2437;
    let wgs = vincenty_inverse_ellipsoid(lat1, lon1, lat2, lon2, &Ellipsoid::WGS84).unwrap();
    let grs = vincenty_inverse_ellipsoid(lat1, lon1, lat2, lon2, &Ellipsoid::GRS80).unwrap();
    let w72 = vincenty_inverse_ellipsoid(lat1, lon1, lat2, lon2, &Ellipsoid::WGS72).unwrap();
    assert!((wgs.distance_m - grs.distance_m).abs() < 0.01);
    assert!((wgs.distance_m - w72.distance_m).abs() < 5.0);
}

#[test]
fn test_vincenty_intermediate_points() {
    use sightline_cot::geo_vincenty::vincenty_intermediate_points;
    let lat1 = 0.0;
    let lon1 = 0.0;
    let lat2 = 0.0;
    let lon2 = 10.0;
    let empty = vincenty_intermediate_points(lat1, lon1, lat2, lon2, 0).unwrap();
    assert!(empty.is_empty());
    let pts = vincenty_intermediate_points(lat1, lon1, lat2, lon2, 3).unwrap();
    assert_eq!(pts.len(), 3);
    for (lat, lon) in &pts {
        assert!(lat.abs() < 1e-6);
        assert!(*lon > 0.0 && *lon < 10.0);
    }
}

#[test]
fn test_vincenty_geodesic_path() {
    use sightline_cot::geo_vincenty::vincenty_geodesic_path;
    let lat1 = 10.0;
    let lon1 = 20.0;
    let lat2 = 10.0;
    let lon2 = 21.0;
    let path = vincenty_geodesic_path(lat1, lon1, lat2, lon2, 25000.0).unwrap();
    assert!(path.len() >= 4);
    assert!((path.first().unwrap().0 - lat1).abs() < 1e-6);
    assert!((path.first().unwrap().1 - lon1).abs() < 1e-6);
    assert!((path.last().unwrap().0 - lat2).abs() < 1e-6);
    assert!((path.last().unwrap().1 - lon2).abs() < 1e-6);
}

#[test]
fn test_crate_root_reexports() {
    use sightline_cot::{
        vincenty_direct, vincenty_direct_ellipsoid, vincenty_geodesic_path,
        vincenty_intermediate_points, vincenty_inverse, vincenty_inverse_ellipsoid, Ellipsoid,
        VincentyDirect, VincentyError, VincentyInverse,
    };
    let inv: Result<VincentyInverse, VincentyError> = vincenty_inverse(0.0, 0.0, 1.0, 1.0);
    assert!(inv.unwrap().distance_m > 0.0);
    let dir: Result<VincentyDirect, VincentyError> = vincenty_direct(0.0, 0.0, 90.0, 1000.0);
    assert!(dir.unwrap().lon_deg > 0.0);
    let _ = vincenty_inverse_ellipsoid(0.0, 0.0, 1.0, 1.0, &Ellipsoid::WGS84);
    let _ = vincenty_direct_ellipsoid(0.0, 0.0, 90.0, 1000.0, &Ellipsoid::WGS84);
    let _ = vincenty_intermediate_points(0.0, 0.0, 1.0, 1.0, 1);
    let _ = vincenty_geodesic_path(0.0, 0.0, 1.0, 1.0, 50000.0);
}

#[test]
fn test_equatorial_near_180_convergence() {
    let inv = vincenty_inverse(0.0, 0.0, 0.0, 179.7).expect("equatorial 179.7 deg");
    assert!((inv.distance_m - 20004112.5).abs() < 1.0);
    assert!((inv.forward_azimuth_deg - 90.0).abs() < 1e-6);
    assert!((inv.reverse_azimuth_deg - 270.0).abs() < 1e-6);
}
