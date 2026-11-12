use sightline_cot::cot::{escape_xml_attr, unescape_xml_attr};

#[test]
fn encoder_strips_illegal_control_chars() {
    assert_eq!(escape_xml_attr("a\u{1}b\u{1f}c"), "abc");
    assert_eq!(escape_xml_attr("a\tb\nc\rd"), "a&#9;b&#10;c&#13;d");
}

#[test]
fn decoder_leaves_illegal_references_undecoded() {
    assert_eq!(unescape_xml_attr("a&#0;b"), "a&#0;b");
    assert_eq!(unescape_xml_attr("&#x1;&#31;"), "&#x1;&#31;");
    assert_eq!(unescape_xml_attr("&#xFFFE;&#xD800;"), "&#xFFFE;&#xD800;");
}

#[test]
fn decoder_rejects_signed_references() {
    assert_eq!(unescape_xml_attr("&#+65;"), "&#+65;");
    assert_eq!(unescape_xml_attr("&#x+41;"), "&#x+41;");
    assert_eq!(unescape_xml_attr("&#65;&#x41;"), "AA");
}
