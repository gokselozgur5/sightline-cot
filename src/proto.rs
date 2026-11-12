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
