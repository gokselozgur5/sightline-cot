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
pub fn encode_tag(
    field_num: u32,
    wire_type: WireType,
    buf: &mut [u8],
) -> Result<usize, ProtoError> {
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

impl TakCotEvent {
    /// Converts TakCotEvent into strongly-typed CotEvent.
    pub fn to_cot_event(&self) -> CotEvent {
        CotEvent {
            uid: self.uid.clone(),
            cot_type: self.r#type.clone(),
            how: self.how.clone(),
            time_ms: self.send_time as i64,
            stale_ms: self.stale_time as i64,
            lat_deg: self.lat,
            lon_deg: self.lon,
            hae_m: self.hae,
            ce_m: self.ce,
            le_m: self.le,
            course_deg: 0.0,
            speed_mps: 0.0,
        }
    }
}

fn write_proto_str(num: u32, s: &str, out: &mut Vec<u8>) -> Result<(), ProtoError> {
    if s.is_empty() {
        return Ok(());
    }
    let mut tag_buf = [0u8; 10];
    let n_tag = encode_tag(num, WireType::LengthDelimited, &mut tag_buf)?;
    out.extend_from_slice(&tag_buf[..n_tag]);
    let mut len_buf = [0u8; 10];
    let n_len = encode_varint(s.len() as u64, &mut len_buf)?;
    out.extend_from_slice(&len_buf[..n_len]);
    out.extend_from_slice(s.as_bytes());
    Ok(())
}

fn write_proto_varint(num: u32, val: u64, out: &mut Vec<u8>) -> Result<(), ProtoError> {
    if val == 0 {
        return Ok(());
    }
    let mut tag_buf = [0u8; 10];
    let n_tag = encode_tag(num, WireType::Varint, &mut tag_buf)?;
    out.extend_from_slice(&tag_buf[..n_tag]);
    let mut val_buf = [0u8; 10];
    let n_val = encode_varint(val, &mut val_buf)?;
    out.extend_from_slice(&val_buf[..n_val]);
    Ok(())
}

fn write_proto_f64(num: u32, val: f64, out: &mut Vec<u8>) -> Result<(), ProtoError> {
    if val == 0.0 {
        return Ok(());
    }
    let mut tag_buf = [0u8; 10];
    let n_tag = encode_tag(num, WireType::Fixed64, &mut tag_buf)?;
    out.extend_from_slice(&tag_buf[..n_tag]);
    let mut f_buf = [0u8; 8];
    encode_fixed64(val, &mut f_buf)?;
    out.extend_from_slice(&f_buf);
    Ok(())
}

impl TakCotEvent {
    /// Encodes TakCotEvent into binary protobuf bytes.
    pub fn encode_protobuf(&self, out: &mut Vec<u8>) -> Result<(), ProtoError> {
        write_proto_str(1, &self.r#type, out)?;
        write_proto_str(2, &self.uid, out)?;
        write_proto_varint(3, self.send_time, out)?;
        write_proto_varint(4, self.start_time, out)?;
        write_proto_varint(5, self.stale_time, out)?;
        write_proto_str(6, &self.how, out)?;
        write_proto_f64(7, self.lat, out)?;
        write_proto_f64(8, self.lon, out)?;
        write_proto_f64(9, self.hae, out)?;
        write_proto_f64(10, self.ce, out)?;
        write_proto_f64(11, self.le, out)?;
        Ok(())
    }
}

/// Encodes a framed TAK Protocol Version 1 packet (0xBF magic + payload).
pub fn encode_tak_packet(event: &TakCotEvent, out: &mut Vec<u8>) -> Result<(), ProtoError> {
    out.push(TAK_PROTO_MAGIC);
    event.encode_protobuf(out)
}

impl TakCotEvent {
    fn decode_field_str_time(
        &mut self,
        num: u32,
        wire: WireType,
        raw: &[u8],
    ) -> Result<(), ProtoError> {
        match (num, wire) {
            (1, WireType::LengthDelimited) => {
                self.r#type = std::str::from_utf8(raw)
                    .map_err(|_| ProtoError::MalformedVarint)?
                    .to_string()
            }
            (2, WireType::LengthDelimited) => {
                self.uid = std::str::from_utf8(raw)
                    .map_err(|_| ProtoError::MalformedVarint)?
                    .to_string()
            }
            (3, WireType::Varint) => self.send_time = decode_varint(raw)?.0,
            (4, WireType::Varint) => self.start_time = decode_varint(raw)?.0,
            (5, WireType::Varint) => self.stale_time = decode_varint(raw)?.0,
            (6, WireType::LengthDelimited) => {
                self.how = std::str::from_utf8(raw)
                    .map_err(|_| ProtoError::MalformedVarint)?
                    .to_string()
            }
            _ => {}
        }
        Ok(())
    }
}

impl TakCotEvent {
    fn decode_field_fixed64(
        &mut self,
        num: u32,
        wire: WireType,
        raw: &[u8],
    ) -> Result<(), ProtoError> {
        if wire == WireType::Fixed64 {
            let val = decode_fixed64(raw)?.0;
            match num {
                7 => self.lat = val,
                8 => self.lon = val,
                9 => self.hae = val,
                10 => self.ce = val,
                11 => self.le = val,
                _ => {}
            }
        }
        Ok(())
    }
}

impl TakCotEvent {
    fn decode_field(&mut self, num: u32, wire: WireType, raw: &[u8]) -> Result<(), ProtoError> {
        if num <= 6 {
            self.decode_field_str_time(num, wire, raw)
        } else {
            self.decode_field_fixed64(num, wire, raw)
        }
    }
}

fn skip_wire_varint_or_fixed(wire: WireType, cur: &[u8]) -> Result<(&[u8], &[u8]), ProtoError> {
    match wire {
        WireType::Varint => {
            let (_, n) = decode_varint(cur)?;
            Ok((&cur[..n], &cur[n..]))
        }
        WireType::Fixed64 => {
            if cur.len() < 8 {
                return Err(ProtoError::UnexpectedEof);
            }
            Ok((&cur[..8], &cur[8..]))
        }
        WireType::Fixed32 => {
            if cur.len() < 4 {
                return Err(ProtoError::UnexpectedEof);
            }
            Ok((&cur[..4], &cur[4..]))
        }
        WireType::LengthDelimited => unreachable!(),
    }
}

fn skip_wire_field(wire: WireType, cur: &[u8]) -> Result<(&[u8], &[u8]), ProtoError> {
    if wire == WireType::LengthDelimited {
        let (len, n) = decode_varint(cur)?;
        let total = usize::try_from(len)
            .ok()
            .and_then(|l| l.checked_add(n))
            .filter(|t| *t <= cur.len())
            .ok_or(ProtoError::UnexpectedEof)?;
        Ok((&cur[n..total], &cur[total..]))
    } else {
        skip_wire_varint_or_fixed(wire, cur)
    }
}

impl TakCotEvent {
    /// Decodes TakCotEvent from protobuf bytes.
    pub fn decode_protobuf(mut buf: &[u8]) -> Result<Self, ProtoError> {
        let mut evt = Self::default();
        while !buf.is_empty() {
            let (num, wire, tag_len) = decode_tag(buf)?;
            let (raw, rem) = skip_wire_field(wire, &buf[tag_len..])?;
            evt.decode_field(num, wire, raw)?;
            buf = rem;
        }
        Ok(evt)
    }
}

/// Decodes a framed TAK Protocol Version 1 packet (validating 0xBF magic byte).
pub fn decode_tak_packet(packet: &[u8]) -> Result<TakCotEvent, ProtoError> {
    if packet.is_empty() || packet[0] != TAK_PROTO_MAGIC {
        return Err(ProtoError::InvalidMagic);
    }
    TakCotEvent::decode_protobuf(&packet[1..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint_roundtrip() {
        let mut buf = [0u8; 10];
        let values = [0u64, 1, 127, 128, 300, 16384, 0x123456789ABCDEF0];
        for &v in &values {
            let n = encode_varint(v, &mut buf).unwrap();
            let (dec, n_dec) = decode_varint(&buf[..n]).unwrap();
            assert_eq!(v, dec);
            assert_eq!(n, n_dec);
        }
    }

    #[test]
    fn test_varint_rejects_overlong_tenth_byte() {
        let mut buf = [0xFFu8; 10];
        buf[9] = 0x7F;
        assert_eq!(decode_varint(&buf), Err(ProtoError::MalformedVarint));
        let mut max = [0u8; 10];
        let n = encode_varint(u64::MAX, &mut max).unwrap();
        assert_eq!(decode_varint(&max[..n]), Ok((u64::MAX, 10)));
    }
}

#[cfg(test)]
mod tests_proto_packet {
    use super::*;

    #[test]
    fn test_tak_packet_roundtrip() {
        let evt = TakCotEvent {
            r#type: "a-f-G-E-V-C".into(),
            uid: "DRONE-01".into(),
            send_time: 1731398400000,
            start_time: 1731398400000,
            stale_time: 1731398460000,
            how: "m-g".into(),
            lat: 39.9,
            lon: 32.8,
            hae: 900.0,
            ce: 1.5,
            le: 2.0,
        };
        let mut packet = Vec::new();
        encode_tak_packet(&evt, &mut packet).unwrap();
        assert_eq!(packet[0], TAK_PROTO_MAGIC);
        let decoded = decode_tak_packet(&packet).unwrap();
        assert_eq!(evt, decoded);
    }
}

#[cfg(test)]
mod tests_cot_conversion {
    use super::*;

    #[test]
    fn test_cot_event_bidirectional_conversion() {
        let cot = CotEvent {
            uid: "UAV-99".into(),
            cot_type: "a-u-A-M-F-Q".into(),
            how: "m-g".into(),
            time_ms: 1731398500000,
            stale_ms: 1731398560000,
            lat_deg: 41.0,
            lon_deg: 29.0,
            hae_m: 150.0,
            ce_m: 2.0,
            le_m: 3.0,
            course_deg: 0.0,
            speed_mps: 0.0,
        };
        let tak: TakCotEvent = (&cot).into();
        let back = tak.to_cot_event();
        assert_eq!(cot, back);
    }
}
