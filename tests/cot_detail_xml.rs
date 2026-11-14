use sightline_cot::cot_detail::{CotContact, CotGroup, CotDetailPayload};

#[test]
fn test_cot_contact_to_xml() {
    let c = CotContact::new("EAGLE-1").with_endpoint("192.168.1.100:4242");
    assert_eq!(
        c.to_xml(),
        r#"<contact callsign="EAGLE-1" endpoint="192.168.1.100:4242"/>"#
    );
}

#[test]
fn test_cot_group_to_xml() {
    let g = CotGroup::new("Blue", "Team Lead");
    assert_eq!(
        g.to_xml(),
        r#"<__group name="Blue" role="Team Lead"/>"#
    );
}
