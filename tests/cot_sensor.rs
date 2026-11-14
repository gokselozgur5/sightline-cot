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
