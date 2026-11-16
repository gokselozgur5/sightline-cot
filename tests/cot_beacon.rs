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

#[test]
fn test_mayday_and_cancel_beacons() {
    let mayday = EmergencyBeacon::new_mayday("AIR-1", "VIPER-01", 36.1, -115.2)
        .expect("mayday");
    let xml = mayday.to_cot_xml();
    assert!(xml.contains("type=\"b-a-o-pan\""));
    assert!(xml.contains("Mayday"));

    let cancel = EmergencyBeacon::new_cancel("AIR-1", "VIPER-01", 36.1, -115.2)
        .expect("cancel");
    let cancel_xml = cancel.to_cot_xml();
    assert!(cancel_xml.contains("cancel=\"true\""));
}

#[test]
fn test_beacon_coordinate_errors() {
    assert!(EmergencyBeacon::new_911("U", "C", 95.0, 0.0).is_err());
    assert!(EmergencyBeacon::new_911("U", "C", -95.0, 0.0).is_err());
    assert!(EmergencyBeacon::new_911("U", "C", 0.0, 185.0).is_err());
    assert!(EmergencyBeacon::new_911("U", "C", f64::NAN, 0.0).is_err());
}
