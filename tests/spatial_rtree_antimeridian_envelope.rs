use sightline_cot::spatial_rtree::SpatialEnvelope;

#[test]
fn polygon_envelope_across_the_antimeridian_contains_its_interior() {
    let poly = [[10.0, 179.0], [10.0, -179.0], [12.0, -179.0], [12.0, 179.0]];
    let env = SpatialEnvelope::from_geodesic_polygon(&poly).unwrap();
    assert!(
        env.contains_point(11.0, 179.5),
        "interior point east of 179 E"
    );
    assert!(
        env.contains_point(11.0, -179.5),
        "interior point west of 179 W"
    );
    assert!(
        !env.contains_point(11.0, 0.0),
        "Greenwich is 179 degrees away"
    );
}
