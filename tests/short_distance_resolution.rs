// A 5 cm offset is 7.8e-9 rad on the mean sphere. acos of a dot product cannot
// resolve angles below about 1.5e-8 rad; the haversine form in the same crate can.
use sightline_cot::geo_polyline_nearest::nearest_point_on_segment;
use sightline_cot::{haversine_distance_m, GreatCirclePath, Waypoint};

const DLAT_5CM: f64 = 0.05 / 111_195.08;

#[test]
fn path_length_resolves_a_five_centimetre_segment() {
    let (a, b) = (
        Waypoint::new(40.0, 30.0),
        Waypoint::new(40.0 + DLAT_5CM, 30.0),
    );
    let hav = haversine_distance_m([a.lat, a.lon], [b.lat, b.lon]);
    assert!((hav - 0.05).abs() < 1e-4);
    let len = GreatCirclePath::from_waypoints(&[a, b])
        .unwrap()
        .total_distance_m();
    assert!((len - 0.05).abs() < 1e-3, "path length {len}");
}

#[test]
fn nearest_point_resolves_a_five_centimetre_offset() {
    let r = nearest_point_on_segment([0.0, 0.0], [0.0, 1.0], [DLAT_5CM, 0.5]);
    assert!(
        (r.distance_m - 0.05).abs() < 1e-3,
        "distance {}",
        r.distance_m
    );
}
