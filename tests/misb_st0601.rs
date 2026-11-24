use sightline_cot::misb_st0601::*;

#[test]
fn test_ber_length_short_and_long() {
    let mut buf = Vec::new();
    encode_ber_length(10, &mut buf);
    assert_eq!(buf, vec![10]);
    let (len, consumed) = decode_ber_length(&buf).unwrap();
    assert_eq!((len, consumed), (10, 1));

    let mut buf2 = Vec::new();
    encode_ber_length(300, &mut buf2);
    assert_eq!(buf2, vec![0x82, 0x01, 0x2C]);
    let (len2, consumed2) = decode_ber_length(&buf2).unwrap();
    assert_eq!((len2, consumed2), (300, 3));
}

#[test]
fn test_ber_length_invalid() {
    assert_eq!(decode_ber_length(&[]), Err(MisbError::BufferTooShort));
    assert_eq!(decode_ber_length(&[0x80]), Err(MisbError::InvalidBerLength));
    assert_eq!(
        decode_ber_length(&[0x85]),
        Err(MisbError::BufferTooShort)
    );
}

#[test]
fn test_ber_oid_tag() {
    let mut buf = Vec::new();
    encode_ber_oid_tag(13, &mut buf);
    assert_eq!(buf, vec![13]);
    let (tag, consumed) = decode_ber_oid_tag(&buf).unwrap();
    assert_eq!((tag, consumed), (13, 1));

    let mut buf2 = Vec::new();
    encode_ber_oid_tag(300, &mut buf2);
    let (tag2, consumed2) = decode_ber_oid_tag(&buf2).unwrap();
    assert_eq!((tag2, consumed2), (300, buf2.len()));
}
