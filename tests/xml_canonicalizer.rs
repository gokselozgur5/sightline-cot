use sightline_cot::xml_canonicalizer::*;

#[test]
fn test_valid_cot_event_validation() {
    let xml = r#"<event version="2.0" uid="TEST-1" type="a-f-G-U-C" how="m-g" time="2026-11-17T04:30:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:35:00.000Z">
        <point lat="37.7749" lon="-122.4194" hae="150.0" ce="5.0" le="2.5"/>
        <detail><track course="45.0" speed="15.0"/></detail>
    </event>"#;

    assert!(validate_cot_xml(xml).is_ok());
}

#[test]
fn test_missing_or_invalid_event_attributes() {
    let no_uid = r#"<event version="2.0" type="a-f-G-U-C" how="m-g" time="2026-11-17T04:30:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:35:00.000Z"><point lat="0.0" lon="0.0" hae="0.0" ce="0.0" le="0.0"/></event>"#;
    assert!(matches!(validate_cot_xml(no_uid), Err(CotValidationError::MissingRequiredAttribute { attribute: "uid", .. })));

    let bad_ver = r#"<event version="1.0" uid="U" type="a-f" how="m-g" time="2026-11-17T04:30:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:35:00.000Z"><point lat="0.0" lon="0.0" hae="0.0" ce="0.0" le="0.0"/></event>"#;
    assert!(matches!(validate_cot_xml(bad_ver), Err(CotValidationError::InvalidVersion(_))));

    let stale_before = r#"<event version="2.0" uid="U" type="a-f" how="m-g" time="2026-11-17T04:35:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:32:00.000Z"><point lat="0.0" lon="0.0" hae="0.0" ce="0.0" le="0.0"/></event>"#;
    assert!(matches!(validate_cot_xml(stale_before), Err(CotValidationError::TimestampOrderViolation { .. })));
