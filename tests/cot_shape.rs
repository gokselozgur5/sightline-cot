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


#[test]
fn test_polygon_xml_generation_and_closed_flag() {
    let vertices = vec![
        CotVertex::new(34.0522, -118.2437).unwrap(),
        CotVertex::new(34.0550, -118.2500).unwrap(),
        CotVertex::new(34.0450, -118.2500).unwrap(),
    ];
    let poly = CotShape::new_polygon(vertices)
        .unwrap()
        .with_color(-16776961)
        .with_fill_color(1073741824);
    assert!(poly.is_closed());
    let xml = poly.to_xml();
    assert!(xml.contains("closed=\"true\""));
    assert!(xml.contains("color=\"-16776961\""));
    assert!(xml.contains("fillColor=\"1073741824\""));
}
