use sightline_cot::cot_beacon::EmergencyType;

#[test]
fn test_emergency_type_from_str() {
    let mayday = "Mayday".parse::<EmergencyType>().unwrap();
    assert_eq!(mayday, EmergencyType::Mayday);
    let custom = "Unknown text".parse::<EmergencyType>().unwrap();
    assert_eq!(custom, EmergencyType::Custom("Unknown text".to_string()));
}
