use sightline_cot::{Egm96Error, Egm96Geoid};

#[test]
fn zero_latitude_step_is_invalid_step() {
    assert_eq!(
        Egm96Geoid::new(0.0, 30.0, vec![]),
        Err(Egm96Error::InvalidStep)
    );
}

#[test]
fn tiny_longitude_step_is_an_error() {
    assert!(Egm96Geoid::new(30.0, 1e-300, vec![]).is_err());
}
