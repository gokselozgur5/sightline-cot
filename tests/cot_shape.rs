use sightline_cot::cot_shape::{CotShape, CotVertex};

#[test]
fn test_polyline_xml_generation() {
    let v1 = CotVertex::new(37.7749, -122.4194).unwrap();
    let v2 = CotVertex::new(37.7752, -122.4178).unwrap();
    let shape = CotShape::new_polyline(vec![v1, v2]).unwrap();
    let xml = shape.to_xml();
    assert!(xml.contains("<shape>"));
    assert!(xml.contains("closed=\"false\""));
}
