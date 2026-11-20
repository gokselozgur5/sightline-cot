// Two points 2 degrees apart across the antimeridian. The tight box around them
// contains the meridian 180 and not the meridian 0 (RFC 7946, 5.2).
use sightline_cot::geo_bbox::GeoBbox;

#[test]
fn bbox_of_points_across_the_antimeridian_is_tight() {
    let bbox = GeoBbox::from_points(&[[0.0, 179.0], [1.0, -179.0]]).unwrap();
    assert!(bbox.contains(0.5, 180.0), "must contain the antimeridian");
    assert!(bbox.contains(0.5, -179.5));
    assert!(
        !bbox.contains(0.5, 0.0),
        "must not contain the prime meridian"
    );
    let c = bbox.center();
    assert!(c[1].abs() > 179.999, "center longitude {}", c[1]);
}
