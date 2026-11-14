use sightline_cot::cot_detail::CotDetailPayload;
use sightline_cot::cot_sensor::CotSensor;

#[test]
fn test_cot_detail_payload_with_sensor() {
    let sensor = CotSensor::new(180.0, 45.0, 1500.0, -15.0).expect("valid sensor");
    let payload = CotDetailPayload::new().with_sensor(sensor);
    assert_eq!(
        payload.to_xml(),
        r#"<detail><sensor azimuth="180.00" fov="45.00" range="1500.00" elevation="-15.00"/></detail>"#
    );
}
