//! Zero-allocation TAK Protobuf wire codec.

use crate::cot::CotEvent;

/// Magic byte framing TAK v1 protobuf messages.
pub const TAK_MAGIC_BYTE: u8 = 0xBF;

/// Errors during protobuf parsing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtoError {
    InvalidMagicByte,
    BufferTooShort,
    VarintOverflow,
    InvalidWireType,
    Utf8Error,
}

/// Encodes an unsigned 64-bit integer into varint bytes.
pub fn encode_varint(mut val: u64, buf: &mut [u8]) -> Result<usize, ProtoError> {
    let mut i = 0;
    while val >= 0x80 {
        if i >= buf.len() {
            return Err(ProtoError::BufferTooShort);
        }
        buf[i] = ((val & 0x7F) | 0x80) as u8;
        val >>= 7;
        i += 1;
    }
    if i >= buf.len() {
        return Err(ProtoError::BufferTooShort);
    }
    buf[i] = (val & 0x7F) as u8;
    Ok(i + 1)
}

/// Decodes a varint from buffer updating offset.
pub fn decode_varint(buf: &[u8], offset: &mut usize) -> Result<u64, ProtoError> {
    let mut result: u64 = 0;
    let mut shift = 0;
    while *offset < buf.len() {
        let b = buf[*offset];
        *offset += 1;
        result |= ((b & 0x7F) as u64) << shift;
        if (b & 0x80) == 0 {
            return Ok(result);
        }
        shift += 7;
        if shift >= 64 {
            return Err(ProtoError::VarintOverflow);
        }
    }
    Err(ProtoError::BufferTooShort)
}

/// Encodes field number and wire type into varint key.
pub fn encode_tag(field_num: u32, wire_type: u8, buf: &mut [u8]) -> Result<usize, ProtoError> {
    let key = ((field_num as u64) << 3) | ((wire_type & 0x07) as u64);
    encode_varint(key, buf)
}

/// Encodes an int64 field as varint.
pub fn encode_i64_field(
    field_num: u32,
    val: i64,
    buf: &mut [u8],
    offset: &mut usize,
) -> Result<(), ProtoError> {
    let tag_len = encode_tag(field_num, 0, &mut buf[*offset..])?;
    *offset += tag_len;
    let len = encode_varint(val as u64, &mut buf[*offset..])?;
    *offset += len;
    Ok(())
}

/// Encodes length-delimited string field.
pub fn encode_string_field(
    field_num: u32,
    s: &str,
    buf: &mut [u8],
    offset: &mut usize,
) -> Result<(), ProtoError> {
    let tag_len = encode_tag(field_num, 2, &mut buf[*offset..])?;
    *offset += tag_len;
    let len_len = encode_varint(s.len() as u64, &mut buf[*offset..])?;
    *offset += len_len;
    if *offset + s.len() > buf.len() {
        return Err(ProtoError::BufferTooShort);
    }
    buf[*offset..*offset + s.len()].copy_from_slice(s.as_bytes());
    *offset += s.len();
    Ok(())
}
