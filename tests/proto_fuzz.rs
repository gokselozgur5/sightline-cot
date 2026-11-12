use sightline_cot::proto::decode_tak_packet;

#[test]
fn huge_length_prefix_returns_error_without_panic() {
    let mut p = vec![0xBF, 0x0A];
    p.extend_from_slice(&[0xFF; 9]);
    p.push(0x01);
    assert!(decode_tak_packet(&p).is_err());
}
