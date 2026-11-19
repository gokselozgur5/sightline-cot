use sightline_cot::cot_beacon::EmergencyBeacon;
use sightline_cot::cot_circle::CotCircle;
use sightline_cot::cot_task::TacticalTask;

#[test]
fn beacon_entities_round_trip() {
    let b = EmergencyBeacon::new_911("U&1", "R&D \"A\"", 34.5, -118.2).unwrap();
    let p = EmergencyBeacon::from_cot_xml(&b.to_cot_xml()).unwrap();
    assert_eq!(p.uid, "U&1");
    assert_eq!(p.callsign, "R&D \"A\"");
}

#[test]
fn circle_entities_round_trip() {
    let c = CotCircle::new(10.0, 20.0, 500.0)
        .unwrap()
        .with_callsign("R&D")
        .with_remarks("a < b");
    let p = CotCircle::parse(&c.to_detail_xml()).unwrap();
    assert_eq!(p.callsign.as_deref(), Some("R&D"));
    assert_eq!(p.remarks.as_deref(), Some("a < b"));
}

#[test]
fn task_entities_round_trip() {
    let t = TacticalTask::new("R&D");
    let p = TacticalTask::parse_task_xml(&t.to_task_xml()).unwrap();
    assert_eq!(p.name, "R&D");
}
