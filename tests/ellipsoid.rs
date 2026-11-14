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

#[test]
fn test_ellipsoid_radii_of_curvature() {
    let e = Ellipsoid::wgs84();
    assert!((e.meridian_radius(0.0) - 6_335_439.327).abs() < 1e-2);
    assert!((e.prime_vertical_radius(0.0) - 6_378_137.0).abs() < 1e-4);
    assert!((e.meridian_radius(90.0) - 6_399_593.626).abs() < 1e-2);
    assert!((e.prime_vertical_radius(90.0) - 6_399_593.626).abs() < 1e-2);
    assert!((e.mean_radius() - 6_371_008.771).abs() < 1e-2);
}

#[test]
fn test_from_a_f_and_other_ellipsoids() {
    let custom = Ellipsoid::from_a_f(6_378_137.0, 1.0 / 298.257_223_563);
    assert_eq!(custom, Ellipsoid::wgs84());
    assert!((GRS80.b() - 6_356_752.314_140).abs() < 1e-4);
    assert!((WGS72.b() - 6_356_750.520_016).abs() < 1e-4);
}
