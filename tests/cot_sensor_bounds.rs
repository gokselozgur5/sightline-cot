use sightline_cot::cot::CotError;
use sightline_cot::cot_sensor::CotSensor;

#[test]
fn test_sensor_bounds_validation() {
    assert_eq!(
        CotSensor::new(-0.01, 45.0, 100.0, 0.0),
        Err(CotError::CoordinateOutOfBounds("azimuth in [0.0, 360.0]"))
    );
    assert_eq!(
        CotSensor::new(360.01, 45.0, 100.0, 0.0),
        Err(CotError::CoordinateOutOfBounds("azimuth in [0.0, 360.0]"))
    );
    assert_eq!(
        CotSensor::new(0.0, 45.0, 100.0, -90.01),
        Err(CotError::CoordinateOutOfBounds(
            "elevation in [-90.0, 90.0]"
        ))
    );
    assert_eq!(
        CotSensor::new(0.0, 45.0, 100.0, 90.01),
        Err(CotError::CoordinateOutOfBounds(
            "elevation in [-90.0, 90.0]"
        ))
    );
}
