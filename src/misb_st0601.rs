//! MISB ST 0601 UAS Datalink Local Set and BER-OID decoder.

/// 16-byte Universal Metadata Key for MISB ST 0601.
pub const MISB_0601_UNIVERSAL_KEY: [u8; 16] = [
    0x06, 0x0E, 0x2B, 0x34, 0x02, 0x0B, 0x01, 0x01, 0x0E, 0x01, 0x03, 0x01, 0x01, 0x00, 0x00, 0x00,
];

/// Errors occurring during MISB ST 0601 and BER-OID decoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MisbError {
    BufferTooShort,
    InvalidBerLength,
    InvalidBerOidTag,
    InvalidUniversalKey,
    InvalidChecksum,
    MissingTag(u8),
    OutOfRange(&'static str),
}

impl core::fmt::Display for MisbError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BufferTooShort => write!(f, "buffer is too short for MISB TLV data"),
            Self::InvalidBerLength => write!(f, "invalid BER length encoding"),
            Self::InvalidBerOidTag => write!(f, "invalid BER-OID tag encoding"),
            Self::InvalidUniversalKey => write!(f, "invalid MISB ST 0601 16-byte universal key"),
            Self::InvalidChecksum => write!(f, "invalid MISB ST 0601 BCC-16 checksum"),
            Self::MissingTag(tag) => write!(f, "required MISB ST 0601 tag {tag} is missing"),
            Self::OutOfRange(msg) => write!(f, "MISB ST 0601 field out of range or sentinel: {msg}"),
        }
    }
}

impl std::error::Error for MisbError {}

/// Decode a BER length field according to ITU-T X.690 / MISB ST 0601 rules.
/// Returns `(length_value, bytes_consumed)`.
pub fn decode_ber_length(bytes: &[u8]) -> Result<(usize, usize), MisbError> {
    if bytes.is_empty() {
        return Err(MisbError::BufferTooShort);
    }
    let first = bytes[0];
    if first < 0x80 {
        return Ok((first as usize, 1));
    }
    if first == 0x80 {
        return Err(MisbError::InvalidBerLength);
    }
    let n = (first & 0x7F) as usize;
    if n > std::mem::size_of::<usize>() {
        return Err(MisbError::InvalidBerLength);
    }
    if bytes.len() < 1 + n {
        return Err(MisbError::BufferTooShort);
    }
    let mut len: usize = 0;
    for &b in &bytes[1..=n] {
        len = len
            .checked_shl(8)
            .ok_or(MisbError::InvalidBerLength)?
            .checked_add(b as usize)
            .ok_or(MisbError::InvalidBerLength)?;
    }
    Ok((len, 1 + n))
}

/// Encode a length value into BER length format.
pub fn encode_ber_length(len: usize, out: &mut Vec<u8>) {
    if len < 0x80 {
        out.push(len as u8);
    } else {
        let be = len.to_be_bytes();
        let first_non_zero = be.iter().position(|&b| b != 0).unwrap_or(be.len() - 1);
        let active_bytes = &be[first_non_zero..];
        out.push(0x80 | (active_bytes.len() as u8));
        out.extend_from_slice(active_bytes);
    }
}

/// Decode a BER-OID tag identifier (e.g. 1-byte, or multi-byte MSB=1 continuation).
/// Returns `(tag_number, bytes_consumed)`.
pub fn decode_ber_oid_tag(bytes: &[u8]) -> Result<(u64, usize), MisbError> {
    if bytes.is_empty() {
        return Err(MisbError::BufferTooShort);
    }
    let mut tag: u64 = 0;
    let mut consumed = 0;
    for &b in bytes {
        consumed += 1;
        if consumed > 10 {
            return Err(MisbError::InvalidBerOidTag);
        }
        tag = tag
            .checked_shl(7)
            .ok_or(MisbError::InvalidBerOidTag)?
            .checked_add((b & 0x7F) as u64)
            .ok_or(MisbError::InvalidBerOidTag)?;
        if (b & 0x80) == 0 {
            return Ok((tag, consumed));
        }
    }
    Err(MisbError::BufferTooShort)
}

/// Encode a tag identifier into BER-OID format.
pub fn encode_ber_oid_tag(tag: u64, out: &mut Vec<u8>) {
    if tag < 0x80 {
        out.push(tag as u8);
    } else {
        let mut buf = [0u8; 10];
        let mut n = 0;
        let mut val = tag;
        while val > 0 {
            buf[n] = (val & 0x7F) as u8;
            val >>= 7;
            n += 1;
        }
        for i in (0..n).rev() {
            let mut b = buf[i];
            if i > 0 {
                b |= 0x80;
            }
            out.push(b);
        }
    }
}

/// Decoded MISB ST 0601 UAS Datalink Local Set frame.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Misb0601Frame {
    /// Tag 2: Precision Time Stamp (microseconds since Unix epoch).
    pub timestamp_micros: Option<u64>,
    /// Tag 5: Platform Heading Angle (0..360 degrees).
    pub platform_heading_deg: Option<f64>,
    /// Tag 6: Platform Pitch Angle (-20..+20 degrees).
    pub platform_pitch_deg: Option<f64>,
    /// Tag 7: Platform Roll Angle (-50..+50 degrees).
    pub platform_roll_deg: Option<f64>,
    /// Tag 13: Sensor Latitude (-90..+90 degrees).
    pub sensor_lat_deg: Option<f64>,
    /// Tag 14: Sensor Longitude (-180..+180 degrees).
    pub sensor_lon_deg: Option<f64>,
    /// Tag 15: Sensor True Altitude (-900..+19000 meters).
    pub sensor_true_alt_m: Option<f64>,
    /// Tag 16: Sensor Horizontal Field of View (0..180 degrees).
    pub sensor_hfov_deg: Option<f64>,
    /// Tag 17: Sensor Vertical Field of View (0..180 degrees).
    pub sensor_vfov_deg: Option<f64>,
    /// Tag 18: Sensor Relative Azimuth Angle (0..360 degrees).
    pub sensor_rel_az_deg: Option<f64>,
    /// Tag 19: Sensor Relative Elevation Angle (-180..+180 degrees).
    pub sensor_rel_el_deg: Option<f64>,
    /// Tag 20: Sensor Relative Roll Angle (0..360 degrees).
    pub sensor_rel_roll_deg: Option<f64>,
    /// Tag 21: Slant Range (0..5,000,000 meters).
    pub slant_range_m: Option<f64>,
    /// Tag 22: Target Width (0..10,000 meters).
    pub target_width_m: Option<f64>,
    /// Tag 23: Frame Center Latitude (-90..+90 degrees).
    pub frame_center_lat_deg: Option<f64>,
    /// Tag 24: Frame Center Longitude (-180..+180 degrees).
    pub frame_center_lon_deg: Option<f64>,
    /// Tag 25: Frame Center Elevation (-900..+19000 meters).
    pub frame_center_elev_m: Option<f64>,
}

fn decode_u16_angle_360(bytes: &[u8]) -> Result<f64, MisbError> {
    if bytes.len() != 2 {
        return Err(MisbError::BufferTooShort);
    }
    let val = u16::from_be_bytes([bytes[0], bytes[1]]);
    Ok((val as f64) * 360.0 / 65535.0)
}

fn encode_u16_angle_360(deg: f64) -> [u8; 2] {
    let clamped = deg.rem_euclid(360.0);
    let val = ((clamped / 360.0) * 65535.0).round() as u16;
    val.to_be_bytes()
}

fn decode_i16_angle(bytes: &[u8], max_deg: f64) -> Result<f64, MisbError> {
    if bytes.len() != 2 {
        return Err(MisbError::BufferTooShort);
    }
    let val = i16::from_be_bytes([bytes[0], bytes[1]]);
    if val == i16::MIN {
        return Err(MisbError::OutOfRange("sentinel or out of range i16"));
    }
    let deg = (val as f64) * max_deg / 32767.0;
    if !(-max_deg..=max_deg).contains(&deg) {
        return Err(MisbError::OutOfRange("angle out of range"));
    }
    Ok(deg)
}

fn encode_i16_angle(deg: f64, max_deg: f64) -> [u8; 2] {
    let clamped = deg.clamp(-max_deg, max_deg);
    let val = ((clamped / max_deg) * 32767.0).round() as i16;
    val.to_be_bytes()
}

fn decode_i32_angle(bytes: &[u8], max_deg: f64) -> Result<f64, MisbError> {
    if bytes.len() != 4 {
        return Err(MisbError::BufferTooShort);
    }
    let val = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    if val == i32::MIN {
        return Err(MisbError::OutOfRange("sentinel or out of range i32"));
    }
    let deg = (val as f64) * max_deg / 2147483647.0;
    if !(-max_deg..=max_deg).contains(&deg) {
        return Err(MisbError::OutOfRange("angle out of range"));
    }
    Ok(deg)
}

fn encode_i32_angle(deg: f64, max_deg: f64) -> [u8; 4] {
    let clamped = deg.clamp(-max_deg, max_deg);
    let val = ((clamped / max_deg) * 2147483647.0).round() as i32;
    val.to_be_bytes()
}





