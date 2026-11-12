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

/// Decodes a protobuf varint from slice, returning (value, bytes_read).
pub fn decode_varint(buf: &[u8]) -> Result<(u64, usize), ProtoError> {
    let mut val = 0u64;
    let mut shift = 0;
    for (i, &b) in buf.iter().enumerate() {
        if i >= 10 {
            return Err(ProtoError::MalformedVarint);
        }
        val |= ((b & 0x7F) as u64) << shift;
        if b & 0x80 == 0 {
            return Ok((val, i + 1));
        }
        shift += 7;
    }
    Err(ProtoError::UnexpectedEof)
}

/// Encodes a protobuf field tag (field_number << 3 | wire_type).
#[inline]
pub fn encode_tag(field_num: u32, wire_type: WireType, buf: &mut [u8]) -> Result<usize, ProtoError> {
    let key = ((field_num as u64) << 3) | (wire_type as u64);
    encode_varint(key, buf)
}

/// Decodes a protobuf field tag returning (field_number, wire_type, bytes_read).
#[inline]
pub fn decode_tag(buf: &[u8]) -> Result<(u32, WireType, usize), ProtoError> {
    let (key, n) = decode_varint(buf)?;
    let wire_raw = (key & 0x07) as u8;
    let field_num = (key >> 3) as u32;
    let wire_type = WireType::from_u8(wire_raw)?;
    Ok((field_num, wire_type, n))
}

/// Decodes a 64-bit IEEE-754 float from little-endian bytes.
#[inline]
pub fn decode_fixed64(buf: &[u8]) -> Result<(f64, usize), ProtoError> {
    if buf.len() < 8 {
        return Err(ProtoError::UnexpectedEof);
    }
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&buf[..8]);
    Ok((f64::from_le_bytes(bytes), 8))
}

/// Encodes a 64-bit IEEE-754 float into little-endian bytes.
#[inline]
pub fn encode_fixed64(val: f64, buf: &mut [u8]) -> Result<usize, ProtoError> {
    if buf.len() < 8 {
        return Err(ProtoError::BufferTooSmall);
    }
    buf[..8].copy_from_slice(&val.to_le_bytes());
    Ok(8)
}

/// Strongly-typed zero-allocation TAK Protobuf CotEvent representation.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TakCotEvent {
    pub r#type: String,
    pub uid: String,
    pub send_time: u64,
    pub start_time: u64,
    pub stale_time: u64,
    pub how: String,
    pub lat: f64,
    pub lon: f64,
    pub hae: f64,
    pub ce: f64,
    pub le: f64,
}

use crate::cot::CotEvent;

impl From<&CotEvent> for TakCotEvent {
    fn from(c: &CotEvent) -> Self {
        Self {
            r#type: c.cot_type.clone(),
            uid: c.uid.clone(),
            send_time: c.time_ms as u64,
            start_time: c.time_ms as u64,
            stale_time: c.stale_ms as u64,
            how: c.how.clone(),
            lat: c.lat_deg,
            lon: c.lon_deg,
            hae: c.hae_m,
            ce: c.ce_m,
            le: c.le_m,
        }
    }
}
