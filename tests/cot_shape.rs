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


#[test]
fn test_shape_and_vertex_error_validation() {
    assert!(CotVertex::new(95.0, 0.0).is_err());
    assert!(CotVertex::new(0.0, -190.0).is_err());
    assert!(CotVertex::new(f64::NAN, 0.0).is_err());

    let v = CotVertex::new(10.0, 20.0).unwrap();
    assert!(CotShape::new_polyline(vec![v]).is_err());
    assert!(CotShape::new_polygon(vec![v, v]).is_err());

    let polyline = CotShape::new_polyline(vec![v, v]).unwrap();
    assert!(polyline.with_stroke_weight(-1.0).is_err());
}


#[test]
fn test_shape_xml_roundtrip_parsing() {
    let xml = r#"<shape><polyline closed="true" color="-65536" fillColor="1073741824" strokeWeight="3.50"><vertex lat="34.052200" lon="-118.243700"/><vertex lat="34.055000" lon="-118.250000"/><vertex lat="34.045000" lon="-118.250000"/></polyline></shape>"#;
    let shape = CotShape::parse(xml).expect("parse shape XML");
    assert!(shape.is_closed());
    assert_eq!(shape.color, Some(-65536));
    assert_eq!(shape.fill_color, Some(1073741824));
    assert_eq!(shape.stroke_weight, Some(3.5));
    assert_eq!(shape.vertices.len(), 3);
}
