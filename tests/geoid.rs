use sightline_cot::geoid::{ConstantGeoid, GeoidModel};

#[test]
fn test_constant_geoid_separation() {
    let geoid = ConstantGeoid::new(30.0);
    assert_eq!(geoid.undulation(45.0, 10.0), Some(30.0));
    assert_eq!(geoid.orthometric_to_ellipsoidal(45.0, 10.0, 100.0), Some(130.0));
    assert_eq!(geoid.ellipsoidal_to_orthometric(45.0, 10.0, 130.0), Some(100.0));
}
