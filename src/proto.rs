//! Zero-allocation TAK Protocol Version 1 (Protobuf CoT) binary wire codec.
//!
//! Provides dependency-free protobuf varint decoding, field parsing,
//! and bidirectional translation to/from CotEvent.

/// Magic byte identifying TAK Protocol Version 1 wire frames.
pub const TAK_PROTO_MAGIC: u8 = 0xBF;

/// Errors during binary TAK protobuf encoding or decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtoError {
    InvalidMagic,
    UnexpectedEof,
    MalformedVarint,
    UnsupportedWireType(u8),
    MissingField(&'static str),
    BufferTooSmall,
}

/// Standard protobuf wire types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireType {
    Varint = 0,
    Fixed64 = 1,
    LengthDelimited = 2,
    Fixed32 = 5,
}

impl WireType {
    /// Parses wire type tag from byte.
    #[inline]
    pub const fn from_u8(val: u8) -> Result<Self, ProtoError> {
        match val {
            0 => Ok(Self::Varint),
            1 => Ok(Self::Fixed64),
            2 => Ok(Self::LengthDelimited),
            5 => Ok(Self::Fixed32),
            other => Err(ProtoError::UnsupportedWireType(other)),
        }
    }
}

/// Encodes an unsigned 64-bit integer into a protobuf varint buffer.
pub fn encode_varint(mut val: u64, buf: &mut [u8]) -> Result<usize, ProtoError> {
    let mut i = 0;
    while val >= 0x80 {
        if i >= buf.len() {
            return Err(ProtoError::BufferTooSmall);
        }
        buf[i] = (val as u8 & 0x7F) | 0x80;
        val >>= 7;
        i += 1;
    }
    if i >= buf.len() {
        return Err(ProtoError::BufferTooSmall);
    }
    buf[i] = val as u8;
    Ok(i + 1)
}
