use sightline_cot::cot_sensor::CotSensor;

#[test]
fn test_sensor_fov_roundtrip() {
    let sensor = CotSensor::new(123.45, 45.0, 1500.0, -15.5).unwrap();
    let xml = sensor.to_xml();
    assert_eq!(
        xml,
        "<sensor azimuth=\"123.45\" fov=\"45.00\" range=\"1500.00\" elevation=\"-15.50\"/>"
    );
    let parsed = CotSensor::parse(&xml).unwrap();
    assert_eq!(parsed, sensor);
}

#[test]
fn test_sensor_rejects_fov_outside_range() {
    assert!(CotSensor::new(0.0, -0.1, 100.0, 0.0).is_err());
    assert!(CotSensor::new(0.0, 360.1, 100.0, 0.0).is_err());
    assert!(CotSensor::new(0.0, 0.0, 100.0, 0.0).is_ok());
    assert!(CotSensor::new(0.0, 360.0, 100.0, 0.0).is_ok());
}
