use sightline_cot::cot::CotEvent;

#[test]
fn cot_event_carries_start_time() {
    let xml = r#"<event version="2.0" uid="u" type="a-f-G" how="m-g" time="2026-02-16T12:00:00.000Z" start="2026-02-16T12:05:00.000Z" stale="2026-02-16T12:10:00.000Z"><point lat="1" lon="1" hae="0" ce="1" le="1"/></event>"#;
    let ev = CotEvent::from_xml(xml).unwrap();
    assert_eq!(ev.start_ms, 1771243500000);
}

#[test]
fn test_cot_parsers_accept_single_quotes_and_strict_attribute_matching() {
    let b_xml = "<event version='2.0' uid='beacon-1' type='b-m-r' how='m-g' time='2026-02-16T12:00:00.000Z' stale='2026-02-16T12:05:00.000Z'><point lat='10.5' lon='20.5' hae='100.0' ce='5.0' le='5.0'/><detail><contact callsign='RESCUE-1'/><emergency cancel='false' type='911 Alert'>SOS</emergency></detail></event>";
    let b = sightline_cot::EmergencyBeacon::from_cot_xml(b_xml)
        .expect("EmergencyBeacon should parse single-quoted XML");
    assert_eq!(b.uid, "beacon-1");
    assert_eq!(b.callsign, "RESCUE-1");

    let s_xml = "<event version='2.0' uid='shape-1' type='u-d-f' how='m-g' time='2026-02-16T12:00:00.000Z' stale='2026-02-16T12:05:00.000Z'><point lat='10.0' lon='20.0'/><detail><shape><polyline closed='true'><vertex lat='10.0' lon='20.0'/><vertex lat='11.0' lon='21.0'/><vertex lat='10.0' lon='21.0'/></polyline></shape></detail></event>";
    let shape =
        sightline_cot::CotShape::parse(s_xml).expect("CotShape should parse single-quoted XML");
    assert_eq!(shape.shape_type, sightline_cot::CotShapeType::Polygon);

    let t_xml = "<task name='patrol' type='route' priority='high' status='in-progress'><point order='1' name='WP1' lat='10.0' lon='20.0' speed='5.5' remarks='check'/></task>";
    let task = sightline_cot::TacticalTask::parse_task_xml(t_xml)
        .expect("TacticalTask should parse single-quoted XML");
    assert_eq!(task.name, "patrol");
    assert_eq!(task.points.len(), 1);

    let c_xml = "<event version='2.0' uid='c1' type='u-d-c' how='m-g' time='2026-02-16T12:00:00.000Z' stale='2026-02-16T12:05:00.000Z'><point lat='10.0' lon='20.0'/><detail><ellipse major='500.0'/><contact callsign='ALPHA'/><remarks>area</remarks></detail></event>";
    let circle =
        sightline_cot::CotCircle::parse(c_xml).expect("CotCircle should parse single-quoted XML");
    assert_eq!(circle.radius_m, 500.0);
    assert_eq!(circle.callsign.as_deref(), Some("ALPHA"));

    let sensor_xml = "<sensor azimuth='45.0' fov='60.0' range='1000.0' elevation='15.0'/>";
    let sensor = sightline_cot::cot_sensor::CotSensor::parse(sensor_xml)
        .expect("CotSensor should parse single-quoted XML");
    assert_eq!(sensor.azimuth_deg, 45.0);
}
