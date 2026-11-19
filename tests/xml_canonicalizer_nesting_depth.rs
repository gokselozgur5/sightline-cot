use sightline_cot::xml_canonicalizer::canonicalize_cot_xml;

#[test]
fn deeply_nested_detail_does_not_overflow_the_stack() {
    let depth = 100_000;
    let mut xml = String::from(
        "<event version=\"2.0\" uid=\"U\" type=\"a-f-G\" how=\"m-g\" time=\"2026-11-17T04:30:00.000Z\" start=\"2026-11-17T04:30:00.000Z\" stale=\"2026-11-17T04:35:00.000Z\"><point lat=\"0.0\" lon=\"0.0\" hae=\"0.0\" ce=\"0.0\" le=\"0.0\"/>",
    );
    xml.push_str(&"<a>".repeat(depth));
    xml.push_str(&"</a>".repeat(depth));
    xml.push_str("</event>");
    let worker = std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(move || canonicalize_cot_xml(&xml).is_ok())
        .unwrap();
    assert!(worker.join().is_ok());
}
