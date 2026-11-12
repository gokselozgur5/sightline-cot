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
