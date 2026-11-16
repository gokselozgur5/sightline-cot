use sightline_cot::cot_beacon::{EmergencyBeacon, EmergencyType};

#[test]
fn test_repro_911_beacon_creation() {
    let beacon = EmergencyBeacon::new_911("UID-911", "MEDEVAC-1", 34.5, -118.2)
        .expect("create 911 beacon");
    assert_eq!(beacon.emergency_type, EmergencyType::Alert911);
    assert_eq!(beacon.callsign, "MEDEVAC-1");
    assert!(!beacon.cancel);
}

#[test]
fn test_911_beacon_xml_roundtrip() {
    let beacon = EmergencyBeacon::new_911("UID-911", "MEDEVAC-1", 34.5, -118.2)
        .expect("beacon")
        .with_details("Critical casualty requesting evac");
    let xml = beacon.to_cot_xml();
    assert!(xml.contains("type=\"b-a-o-tbl\""));
    assert!(xml.contains("callsign=\"MEDEVAC-1\""));
    assert!(xml.contains("Critical casualty requesting evac"));
    let parsed = EmergencyBeacon::from_cot_xml(&xml).expect("parse XML");
    assert_eq!(parsed.uid, "UID-911");
    assert_eq!(parsed.callsign, "MEDEVAC-1");
    assert_eq!(parsed.emergency_type, EmergencyType::Alert911);
}
