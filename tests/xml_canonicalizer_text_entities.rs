use sightline_cot::xml_canonicalizer::canonicalize_cot_xml;

const XML: &str = "<event version=\"2.0\" uid=\"U\" type=\"a-f-G\" how=\"m-g\" \
time=\"2026-11-17T04:30:00.000Z\" start=\"2026-11-17T04:30:00.000Z\" \
stale=\"2026-11-17T04:35:00.000Z\">\
<point lat=\"0.0\" lon=\"0.0\" hae=\"0.0\" ce=\"0.0\" le=\"0.0\"/>\
<detail><remarks>A &amp; B &lt;C&gt;</remarks></detail></event>";

#[test]
fn text_entities_are_not_escaped_twice() {
    let canon = canonicalize_cot_xml(XML).unwrap();
    assert!(
        canon.contains("<remarks>A &amp; B &lt;C&gt;</remarks>"),
        "{canon}"
    );
}

#[test]
fn canonicalization_is_idempotent() {
    let once = canonicalize_cot_xml(XML).unwrap();
    let twice = canonicalize_cot_xml(&once).unwrap();
    assert_eq!(once, twice);
}
