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
