use sightline_cot::proto::{decode_tak_packet, ProtoError};

fn packet_with_uid(bytes: &[u8]) -> Vec<u8> {
    let mut p = vec![0xBF, 0x12, bytes.len() as u8];
    p.extend_from_slice(bytes);
    p
}

fn assert_invalid(bytes: &[u8]) {
    assert_eq!(
        decode_tak_packet(&packet_with_uid(bytes)).unwrap_err(),
        ProtoError::InvalidUtf8
    );
}

#[test]
fn invalid_continuation_byte() {
    assert_invalid(&[b'a', 0xC3, 0x28]);
}

#[test]
fn truncated_multibyte_sequence() {
    assert_invalid(&[b'a', 0xE2, 0x82]);
}

#[test]
fn overlong_encoding() {
    assert_invalid(&[0xC0, 0xAF]);
}
