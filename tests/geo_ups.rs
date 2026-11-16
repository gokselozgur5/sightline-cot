use sightline_cot::geo_ups::{UpsCoord, UpsHemisphere};

#[test]
fn test_ups_coord_types() {
    let coord = UpsCoord::new(2000000.0, 2000000.0, UpsHemisphere::North);
    assert_eq!(coord.easting, 2000000.0);
    assert_eq!(coord.northing, 2000000.0);
    assert_eq!(coord.hemisphere, UpsHemisphere::North);
}
