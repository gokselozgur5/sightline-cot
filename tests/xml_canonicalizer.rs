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
    assert!(matches!(
        validate_cot_xml(no_uid),
        Err(CotValidationError::MissingRequiredAttribute {
            attribute: "uid",
            ..
        })
    ));

    let bad_ver = r#"<event version="1.0" uid="U" type="a-f" how="m-g" time="2026-11-17T04:30:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:35:00.000Z"><point lat="0.0" lon="0.0" hae="0.0" ce="0.0" le="0.0"/></event>"#;
    assert!(matches!(
        validate_cot_xml(bad_ver),
        Err(CotValidationError::InvalidVersion(_))
    ));

    let stale_before = r#"<event version="2.0" uid="U" type="a-f" how="m-g" time="2026-11-17T04:35:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:32:00.000Z"><point lat="0.0" lon="0.0" hae="0.0" ce="0.0" le="0.0"/></event>"#;
    assert!(matches!(
        validate_cot_xml(stale_before),
        Err(CotValidationError::TimestampOrderViolation { .. })
    ));
}

#[test]
fn test_point_coordinate_bounds_and_errors() {
    let lat_oob = r#"<event version="2.0" uid="U" type="a-f" how="m-g" time="2026-11-17T04:30:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:35:00.000Z"><point lat="95.0" lon="0.0" hae="0.0" ce="0.0" le="0.0"/></event>"#;
    assert!(matches!(
        validate_cot_xml(lat_oob),
        Err(CotValidationError::CoordinateOutOfBounds {
            coordinate: "lat",
            ..
        })
    ));

    let lon_oob = r#"<event version="2.0" uid="U" type="a-f" how="m-g" time="2026-11-17T04:30:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:35:00.000Z"><point lat="0.0" lon="-190.0" hae="0.0" ce="0.0" le="0.0"/></event>"#;
    assert!(matches!(
        validate_cot_xml(lon_oob),
        Err(CotValidationError::CoordinateOutOfBounds {
            coordinate: "lon",
            ..
        })
    ));

    let neg_ce = r#"<event version="2.0" uid="U" type="a-f" how="m-g" time="2026-11-17T04:30:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:35:00.000Z"><point lat="0.0" lon="0.0" hae="0.0" ce="-1.0" le="0.0"/></event>"#;
    assert!(matches!(
        validate_cot_xml(neg_ce),
        Err(CotValidationError::NegativeValueError { field: "ce", .. })
    ));
}

#[test]
fn test_malformed_xml_and_unclosed_tags() {
    let unclosed = r#"<event version="2.0" uid="U" type="a-f" how="m-g" time="2026-11-17T04:30:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:35:00.000Z"><point lat="0.0" lon="0.0" hae="0.0" ce="0.0" le="0.0"/>"#;
    assert!(matches!(
        validate_cot_xml(unclosed),
        Err(CotValidationError::UnclosedTag(_))
    ));

    let dup_attr = r#"<event version="2.0" uid="U" uid="U2" type="a-f" how="m-g" time="2026-11-17T04:30:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:35:00.000Z"><point lat="0.0" lon="0.0" hae="0.0" ce="0.0" le="0.0"/></event>"#;
    assert!(matches!(
        validate_cot_xml(dup_attr),
        Err(CotValidationError::DuplicateAttribute { .. })
    ));
}

#[test]
fn test_canonicalization_sorting_and_namespaces() {
    let xml = r#"<event stale="2026-11-17T04:35:00.000Z" start="2026-11-17T04:30:00.000Z" time="2026-11-17T04:30:00.000Z" how="m-g" type="a-f-G-U-C" uid="U" version="2.0" xmlns:cot="http://cot.example.com" xmlns="http://default.example.com">
        <!-- sample comment -->
        <point le="0.0" ce="0.0" hae="10.0" lon="20.0" lat="10.0"/>
    </event>"#;

    let canon = canonicalize_cot_xml(xml).expect("canonicalization");
    assert!(!canon.contains("sample comment"));
    assert!(canon.starts_with("<event xmlns=\"http://default.example.com\" xmlns:cot=\"http://cot.example.com\" how=\"m-g\" stale=\"2026-11-17T04:35:00.000Z\" start=\"2026-11-17T04:30:00.000Z\" time=\"2026-11-17T04:30:00.000Z\" type=\"a-f-G-U-C\" uid=\"U\" version=\"2.0\">"));
    assert!(canon.contains("<point ce=\"0.0\" hae=\"10.0\" lat=\"10.0\" le=\"0.0\" lon=\"20.0\"/>"));
}

#[test]
fn test_redundant_namespace_stripping() {
    let xml = r#"<event version="2.0" uid="U" type="a-f" how="m-g" time="2026-11-17T04:30:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:35:00.000Z" xmlns:cot="http://cot.example.com">
        <point lat="0.0" lon="0.0" hae="0.0" ce="0.0" le="0.0"/>
        <detail>
            <item xmlns:cot="http://cot.example.com" name="test"/>
        </detail>
    </event>"#;

    let canon = canonicalize_cot_xml(xml).expect("canonicalization");
    assert!(canon.contains("<item name=\"test\"/>"));
    assert!(!canon.contains("<item xmlns:cot"));
}

#[test]
fn test_repro_preserve_comments_when_disabled() {
    let xml = r#"<event version="2.0" uid="U" type="a-f" how="m-g" time="2026-11-17T04:30:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:35:00.000Z"><point lat="0.0" lon="0.0" hae="0.0" ce="0.0" le="0.0"/><detail><!-- note --></detail></event>"#;
    let opts = CanonicalizeOptions {
        remove_comments: false,
        ..Default::default()
    };
    let canon = CotXmlCanonicalizer::canonicalize_with_options(xml, &opts).expect("canonicalize");
    assert!(canon.contains("<!-- note -->"));
}

#[test]
fn test_repro_reject_non_whitespace_text_outside_root() {
    let xml_before = r#"junk<event version="2.0" uid="U" type="a-f" how="m-g" time="2026-11-17T04:30:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:35:00.000Z"><point lat="0.0" lon="0.0" hae="0.0" ce="0.0" le="0.0"/></event>"#;
    assert!(matches!(
        validate_cot_xml(xml_before),
        Err(CotValidationError::MalformedXml(_))
    ));

    let xml_after = r#"<event version="2.0" uid="U" type="a-f" how="m-g" time="2026-11-17T04:30:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:35:00.000Z"><point lat="0.0" lon="0.0" hae="0.0" ce="0.0" le="0.0"/></event>trailing"#;
    assert!(matches!(
        validate_cot_xml(xml_after),
        Err(CotValidationError::MalformedXml(_))
    ));
}
