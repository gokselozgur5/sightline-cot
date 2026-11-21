use sightline_cot::cot_beacon::EmergencyBeacon;

#[test]
fn beacon_point_errors_survive_a_round_trip() {
    let mut beacon = EmergencyBeacon::new_911("UID-911", "MEDEVAC-1", 34.5, -118.2).unwrap();
    beacon.ce_m = 7.0;
    beacon.le_m = 25.0;
    let parsed = EmergencyBeacon::from_cot_xml(&beacon.to_cot_xml()).unwrap();
    assert_eq!(parsed.ce_m, 7.0);
    assert_eq!(parsed.le_m, 25.0);
}
