use sightline_cot::xml_canonicalizer::canonicalize_cot_xml;

#[test]
fn control_character_references_do_not_become_raw_control_bytes() {
    let xml = "<event version=\"2.0\" uid=\"A&#1;B&#0;C\" type=\"a-f-G\" how=\"m-g\" \
        time=\"2026-11-17T04:30:00.000Z\" start=\"2026-11-17T04:30:00.000Z\" \
        stale=\"2026-11-17T04:35:00.000Z\">\
        <point lat=\"0.0\" lon=\"0.0\" hae=\"0.0\" ce=\"0.0\" le=\"0.0\"/></event>";
    if let Ok(canon) = canonicalize_cot_xml(xml) {
        assert!(
            !canon.contains('\u{1}') && !canon.contains('\u{0}'),
            "{canon:?}"
        );
    }
}
