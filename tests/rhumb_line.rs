use sightline_cot::{
    cross_track_distance, cross_track_distance_m, interpolate_by_count, rhumb_bearing,
    rhumb_destination, rhumb_distance, rhumb_distance_and_bearing, vincenty_direct,
    vincenty_direct_ellipsoid, Ellipsoid, LocalFrame, RhumbLineError, RhumbWaypoint,
};

#[test]
fn test_interpolate_by_count_rejects_count_exceeding_max_sample_count() {
    let p1 = RhumbWaypoint::new(10.0, 10.0);
    let p2 = RhumbWaypoint::new(10.0, 20.0);
    assert_eq!(
        interpolate_by_count(&p1, &p2, usize::MAX),
        Err(RhumbLineError::InvalidParameter)
    );
}

#[test]
fn test_crate_root_enu_local_frame_three_arg_roundtrip() {
    let frame = LocalFrame::new(37.7749, -122.4194, 50.0).unwrap();
    let enu = frame.geodetic_to_enu(37.7849, -122.4094, 150.0).unwrap();
    let (lat, lon, hae) = frame.enu_to_geodetic(enu).unwrap();
    assert!((lat - 37.7849).abs() < 1e-7);
    assert!((lon - (-122.4094)).abs() < 1e-7);
    assert!((hae - 150.0).abs() < 1e-3);
}

#[test]
fn test_unified_rhumb_crosstrack_and_vincenty_direct_solvers() {
    let start = [12.0, 24.0];
    let dest = rhumb_destination(start, 60.0, 50_000.0);
    let (wp_dist, wp_brg) = rhumb_distance_and_bearing(
        &RhumbWaypoint::new(start[0], start[1]),
        &RhumbWaypoint::new(dest[0], dest[1]),
        sightline_cot::rhumb_line::EARTH_RADIUS_M,
    )
    .unwrap();
    assert!((rhumb_distance(start, dest) - wp_dist).abs() < 1e-9);
    assert!((rhumb_bearing(start, dest) - wp_brg).abs() < 1e-9);

    let xt_arr = cross_track_distance_m([0.0, 0.0], [0.0, 10.0], [-1.0, 5.0]);
    let xt_tup = cross_track_distance((0.0, 0.0), (0.0, 10.0), (-1.0, 5.0));
    assert!((xt_arr - xt_tup).abs() < 1e-9);

    let vd_wgs = vincenty_direct(35.0, 33.0, 45.0, 25_000.0).unwrap();
    let vd_ell = vincenty_direct_ellipsoid(35.0, 33.0, 45.0, 25_000.0, &Ellipsoid::WGS84).unwrap();
    assert_eq!(vd_wgs, vd_ell);
}
