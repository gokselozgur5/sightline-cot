use sightline_cot::proto::{decode_tak_packet, encode_tak_packet, TakCotEvent};

#[test]
fn multibyte_text_roundtrips() {
    let evt = TakCotEvent {
        uid: "d\u{00fc}\u{20ac}\u{1F680}".to_string(),
        r#type: "a-f-G".to_string(),
        how: "m-g".to_string(),
        ..Default::default()
    };
    let mut buf = Vec::new();
    encode_tak_packet(&evt, &mut buf).unwrap();
    assert_eq!(decode_tak_packet(&buf).unwrap().uid, evt.uid);
}
