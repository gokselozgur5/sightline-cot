use sightline_cot::cot_sensor::CotSensor;

#[test]
fn test_sensor_bounds_validation() {
    assert!(CotSensor::new(-0.01, 45.0, 100.0, 0.0).is_err());
    assert!(CotSensor::new(360.01, 45.0, 100.0, 0.0).is_err());
    assert!(CotSensor::new(0.0, 45.0, 100.0, -90.01).is_err());
    assert!(CotSensor::new(0.0, 45.0, 100.0, 90.01).is_err());
}
