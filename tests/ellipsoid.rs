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

#[test]
fn test_ellipsoid_parameter_accessors() {
    let e = Ellipsoid::wgs84();
    assert_eq!(e.a(), 6_378_137.0);
    assert_eq!(e.inv_f(), 298.257_223_563);
    assert!((e.b() - 6_356_752.314_245).abs() < 1e-4);
    assert!((e.f() - 1.0 / 298.257_223_563).abs() < 1e-12);
    assert!((e.e2() - 0.006_694_379_990_14).abs() < 1e-12);
    assert!((e.e() - 0.081_819_190_842).abs() < 1e-10);
    assert!((e.e_prime2() - 0.006_739_496_742).abs() < 1e-10);
}
