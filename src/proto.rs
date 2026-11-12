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

/// Encodes fixed64 64-bit float field.
pub fn encode_f64_field(
    field_num: u32,
    val: f64,
    buf: &mut [u8],
    offset: &mut usize,
) -> Result<(), ProtoError> {
    let tag_len = encode_tag(field_num, 1, &mut buf[*offset..])?;
    *offset += tag_len;
    if *offset + 8 > buf.len() {
        return Err(ProtoError::BufferTooShort);
    }
    let bits = val.to_bits();
    buf[*offset..*offset + 8].copy_from_slice(&bits.to_le_bytes());
    *offset += 8;
    Ok(())
}

/// TAK Protobuf Message wrapper around CotEvent.
#[derive(Clone, Debug, PartialEq)]
pub struct TakMessage {
    pub cot_event: CotEvent,
}

impl TakMessage {
    /// Creates a new TakMessage from CotEvent.
    pub fn new(cot_event: CotEvent) -> Self {
        Self { cot_event }
    }
}

impl TakMessage {
    /// Encodes framed message with 0xBF magic byte prefix.
    pub fn encode_framed(&self, buf: &mut [u8]) -> Result<usize, ProtoError> {
        if buf.is_empty() {
            return Err(ProtoError::BufferTooShort);
        }
        buf[0] = TAK_MAGIC_BYTE;
        let mut off = 1;
        self.encode_payload(buf, &mut off)?;
        Ok(off)
    }
}

impl TakMessage {
    fn encode_payload(&self, buf: &mut [u8], off: &mut usize) -> Result<(), ProtoError> {
        encode_string_field(1, &self.cot_event.uid, buf, off)?;
        encode_string_field(2, &self.cot_event.cot_type, buf, off)?;
        encode_string_field(3, &self.cot_event.how, buf, off)?;
        encode_i64_field(4, self.cot_event.time_ms, buf, off)?;
        encode_i64_field(5, self.cot_event.stale_ms, buf, off)?;
        encode_f64_field(6, self.cot_event.lat_deg, buf, off)?;
        encode_f64_field(7, self.cot_event.lon_deg, buf, off)?;
        encode_f64_field(8, self.cot_event.hae_m, buf, off)?;
        Ok(())
    }
}

impl TakMessage {
    /// Decodes framed message validating 0xBF magic prefix.
    pub fn decode_framed(buf: &[u8]) -> Result<Self, ProtoError> {
        if buf.is_empty() {
            return Err(ProtoError::BufferTooShort);
        }
        if buf[0] != TAK_MAGIC_BYTE {
            return Err(ProtoError::InvalidMagicByte);
        }
        Self::decode_payload(&buf[1..])
    }
}

impl TakMessage {
    fn decode_payload(buf: &[u8]) -> Result<Self, ProtoError> {
        let mut off = 0;
        let mut uid = String::new();
        let mut cot_type = String::new();
        let mut how = String::new();
        let mut time_ms = 0i64;
        let mut stale_ms = 0i64;
        let mut lat_deg = 0.0f64;
        let mut lon_deg = 0.0f64;
        let mut hae_m = 0.0f64;
        Self::decode_fields(
            buf,
            &mut off,
            &mut uid,
            &mut cot_type,
            &mut how,
            &mut time_ms,
            &mut stale_ms,
            &mut lat_deg,
            &mut lon_deg,
            &mut hae_m,
        )?;
        let evt = CotEvent {
            uid,
            cot_type,
            how,
            time_ms,
            stale_ms,
            lat_deg,
            lon_deg,
            hae_m,
            ce_m: 5.0,
            le_m: 10.0,
            course_deg: 0.0,
            speed_mps: 0.0,
        };
        Ok(Self { cot_event: evt })
    }
}
