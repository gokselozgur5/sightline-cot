use sightline_cot::geo_midpoint::geodesic_midpoint_ellipsoid;
use sightline_cot::vincenty_inverse;

#[test]
fn coincident_points_on_the_antimeridian_have_zero_distance() {
    let inv = vincenty_inverse(10.0, 180.0, 10.0, -180.0);
    assert!(
        matches!(inv, Ok(r) if r.distance_m.abs() < 1e-6),
        "got {inv:?}"
    );
}

#[test]
fn ellipsoid_midpoint_of_coincident_antimeridian_points_is_the_point() {
    let mid = geodesic_midpoint_ellipsoid(10.0, 180.0, 10.0, -180.0, 0.5);
    assert!(
        matches!(mid, Ok((lat, lon)) if (lat - 10.0).abs() < 1e-9 && lon.abs() > 179.999),
        "got {mid:?}"
    );
}
