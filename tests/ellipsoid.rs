use sightline_cot::ellipsoid::{Ellipsoid, GRS80, WGS72, WGS84};

#[test]
fn test_ellipsoid_constants() {
    assert_eq!(WGS84.a, 6_378_137.0);
    assert_eq!(WGS84.inv_f, 298.257_223_563);
    assert_eq!(GRS80.a, 6_378_137.0);
    assert_eq!(GRS80.inv_f, 298.257_222_101);
    assert_eq!(WGS72.a, 6_378_135.0);
    assert_eq!(WGS72.inv_f, 298.26);
}

#[test]
fn test_ellipsoid_constructors() {
    assert_eq!(Ellipsoid::wgs84(), WGS84);
    assert_eq!(Ellipsoid::grs80(), GRS80);
    assert_eq!(Ellipsoid::wgs72(), WGS72);
}
