//! Integration tests for geodesic midpoint.

use sightline_cot::geo_midpoint::geodesic_midpoint;

#[test]
fn test_geodesic_midpoint_identical_and_equator() {
    let (lat, lon) = geodesic_midpoint(10.0, 20.0, 10.0, 20.0).unwrap();
    assert!((lat - 10.0).abs() < 1e-6);
    assert!((lon - 20.0).abs() < 1e-6);

    let (m_lat, m_lon) = geodesic_midpoint(0.0, 10.0, 0.0, 20.0).unwrap();
    assert!((m_lat - 0.0).abs() < 1e-6);
    assert!((m_lon - 15.0).abs() < 1e-6);
}

#[test]
fn test_geodesic_midpoint_antimeridian() {
    let (lat, lon) = geodesic_midpoint(0.0, 170.0, 0.0, -170.0).unwrap();
    assert!(lat.abs() < 1e-6);
    assert!((lon.abs() - 180.0).abs() < 1e-6);
}

#[test]
fn test_geodesic_midpoint_errors() {
    use sightline_cot::geo_midpoint::MidpointError;

    assert_eq!(
        geodesic_midpoint(0.0, 0.0, 0.0, 180.0),
        Err(MidpointError::AntipodalPoints)
    );
    assert_eq!(
        geodesic_midpoint(91.0, 0.0, 0.0, 0.0),
        Err(MidpointError::InvalidLatitude)
    );
    assert_eq!(
        geodesic_midpoint(0.0, 181.0, 0.0, 0.0),
        Err(MidpointError::InvalidLongitude)
    );
}

#[test]
fn test_geodesic_midpoint_ellipsoid_failing_repro() {
    use sightline_cot::geo_midpoint::geodesic_midpoint_ellipsoid;

    let res = geodesic_midpoint_ellipsoid(0.0, 10.0, 0.0, 20.0, 0.5);
    assert!(res.is_ok());
}

#[test]
fn test_geodesic_midpoint_ellipsoid_endpoints_and_equidistant() {
    use sightline_cot::geo_midpoint::geodesic_midpoint_ellipsoid;
    use sightline_cot::geo_vincenty::vincenty_inverse;

    let (lat1, lon1) = (10.0, 20.0);
    let (lat2, lon2) = (12.0, 25.0);

    let (start_lat, start_lon) = geodesic_midpoint_ellipsoid(lat1, lon1, lat2, lon2, 0.0).unwrap();
    assert!((start_lat - lat1).abs() < 1e-9);
    assert!((start_lon - lon1).abs() < 1e-9);

    let (end_lat, end_lon) = geodesic_midpoint_ellipsoid(lat1, lon1, lat2, lon2, 1.0).unwrap();
    assert!((end_lat - lat2).abs() < 1e-9);
    assert!((end_lon - lon2).abs() < 1e-9);

    let (mid_lat, mid_lon) = geodesic_midpoint_ellipsoid(lat1, lon1, lat2, lon2, 0.5).unwrap();
    let d1 = vincenty_inverse(lat1, lon1, mid_lat, mid_lon)
        .unwrap()
        .distance_m;
    let d2 = vincenty_inverse(mid_lat, mid_lon, lat2, lon2)
        .unwrap()
        .distance_m;
    assert!((d1 - d2).abs() < 1e-6);
}
