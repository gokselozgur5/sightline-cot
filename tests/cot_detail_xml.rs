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

#[test]
fn test_cot_detail_payload_full() {
    let p = CotDetailPayload::new()
        .with_contact(CotContact::new("HAWK-2"))
        .with_group(CotGroup::new("Alpha", "Recon"));
    assert_eq!(
        p.to_xml(),
        r#"<detail><contact callsign="HAWK-2"/><__group name="Alpha" role="Recon"/></detail>"#
    );
}

#[test]
fn test_cot_detail_payload_empty() {
    let p = CotDetailPayload::new();
    assert_eq!(p.to_xml(), "<detail/>");
}
