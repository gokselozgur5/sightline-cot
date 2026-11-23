//! NATO STANAG 4609 / MISB ST 0601 KLV metadata timestamp translator.

/// Errors occurring during STANAG 4609 timestamp parsing and translation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StanagError {
    NegativeTimestamp,
    InvalidKlvTag(u8),
    InvalidKlvLength(usize),
    BufferTooShort,
    InvalidIso8601(String),
    InvalidChecksum,
}

impl std::fmt::Display for StanagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NegativeTimestamp => write!(f, "timestamp cannot be negative"),
            Self::InvalidKlvTag(t) => {
                write!(f, "invalid KLV tag: 0x{t:02X}, expected Tag 2 (0x02)")
            }
            Self::InvalidKlvLength(len) => write!(f, "invalid KLV length: {len}, expected 8"),
            Self::BufferTooShort => write!(f, "buffer is too short for KLV timestamp"),
            Self::InvalidIso8601(s) => write!(f, "invalid ISO 8601 date string: {s}"),
            Self::InvalidChecksum => write!(f, "invalid ST 0601 checksum"),
        }
    }
}

impl std::error::Error for StanagError {}

/// Precision timestamp in microseconds since Unix epoch (MISB ST 0601 Tag 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StanagTimestamp {
    micros: u64,
}

impl StanagTimestamp {
    /// Create a timestamp from elapsed microseconds since Unix epoch.
    pub const fn from_micros(micros: u64) -> Self {
        Self { micros }
    }

    /// Return the raw microsecond value since Unix epoch.
    pub const fn as_micros(&self) -> u64 {
        self.micros
    }
}

impl StanagTimestamp {
    /// Create a timestamp from elapsed milliseconds since Unix epoch (CoT format).
    pub fn from_millis(millis: i64) -> Result<Self, StanagError> {
        if millis < 0 {
            return Err(StanagError::NegativeTimestamp);
        }
        let micros = (millis as u64)
            .checked_mul(1_000)
            .ok_or_else(|| StanagError::InvalidIso8601("timestamp overflow".to_string()))?;
        Ok(Self::from_micros(micros))
    }

    /// Convert timestamp to Unix epoch milliseconds used in CoT event attributes.
    pub const fn to_cot_millis(&self) -> i64 {
        (self.micros / 1_000) as i64
    }

    /// Convert timestamp to fractional seconds since Unix epoch.
    pub fn to_secs_f64(&self) -> f64 {
        (self.micros as f64) / 1_000_000.0
    }
}

impl StanagTimestamp {
    /// Create a timestamp from 8 big-endian bytes (MISB ST 0601 wire format).
    pub const fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self::from_micros(u64::from_be_bytes(bytes))
    }

    /// Serialize timestamp to 8 big-endian bytes.
    pub const fn to_be_bytes(&self) -> [u8; 8] {
        self.micros.to_be_bytes()
    }
}

/// MISB ST 0601 Tag for Precision Time Stamp (PTS).
pub const MISB_TAG_PRECISION_TIME_STAMP: u8 = 0x02;

/// Length of Precision Time Stamp field in bytes.
pub const MISB_LEN_PRECISION_TIME_STAMP: u8 = 0x08;

impl StanagTimestamp {
    /// Format Tag 2 KLV TLV element (Tag 0x02, Len 0x08, 8 bytes value).
    pub fn to_klv_tag2(&self) -> [u8; 10] {
        let mut out = [0u8; 10];
        out[0] = MISB_TAG_PRECISION_TIME_STAMP;
        out[1] = MISB_LEN_PRECISION_TIME_STAMP;
        out[2..10].copy_from_slice(&self.to_be_bytes());
        out
    }
}

impl StanagTimestamp {
    /// Parse Tag 2 timestamp from a KLV byte slice (tag, length, 8-byte value).
    pub fn from_klv_slice(bytes: &[u8]) -> Result<Self, StanagError> {
        if bytes.len() < 10 {
            return Err(StanagError::BufferTooShort);
        }
        let tag = bytes[0];
        if tag != MISB_TAG_PRECISION_TIME_STAMP {
            return Err(StanagError::InvalidKlvTag(tag));
        }
        let len = bytes[1] as usize;
        if len != (MISB_LEN_PRECISION_TIME_STAMP as usize) {
            return Err(StanagError::InvalidKlvLength(len));
        }
        let mut val = [0u8; 8];
        val.copy_from_slice(&bytes[2..10]);
        Ok(Self::from_be_bytes(val))
    }
}

impl StanagTimestamp {
    /// Format timestamp as ISO 8601 UTC string (millisecond precision).
    pub fn to_iso8601_utc(&self) -> String {
        crate::cot::format_iso8601_utc(self.to_cot_millis())
    }

    /// Format timestamp as ISO 8601 UTC string with full microsecond precision.
    pub fn to_iso8601_micros(&self) -> String {
        let base = self.to_iso8601_utc();
        let rem_micros = (self.micros % 1_000_000) as u32;
        format!("{}.{:06}Z", &base[..19], rem_micros)
    }
}

impl StanagTimestamp {
    /// Parse an ISO 8601 UTC timestamp string into a StanagTimestamp.
    pub fn from_iso8601(s: &str) -> Result<Self, StanagError> {
        let ms = crate::cot::parse_iso8601_utc(s)
            .map_err(|e| StanagError::InvalidIso8601(e.to_string()))?;
        if ms < 0 {
            return Err(StanagError::NegativeTimestamp);
        }
        let mut micros = (ms as u64) * 1_000;
        let s = s.trim();
        if let Some(dot_idx) = s.find('.') {
            let frac = s[dot_idx + 1..].trim_end_matches('Z');
            let digits: String = frac.chars().take_while(|c| c.is_ascii_digit()).collect();
            let padded = format!("{:0<6}", digits);
            if let Ok(extra_us) = padded[3..6].parse::<u64>() {
                micros += extra_us;
            }
        }
        Ok(Self::from_micros(micros))
    }
}

/// Compute the MISB ST 0601 16-bit running sum checksum over a byte slice.
pub fn compute_st0601_checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    for (i, &b) in data.iter().enumerate() {
        if i % 2 == 0 {
            sum += (b as u32) << 8;
        } else {
            sum += b as u32;
        }
    }
    (sum & 0xFFFF) as u16
}

/// 16-byte Universal Metadata Key for MISB ST 0601 UAS Datalink Local Set.
pub const MISB_0601_UNIVERSAL_KEY: [u8; 16] = [
    0x06, 0x0E, 0x2B, 0x34, 0x02, 0x0B, 0x01, 0x01, 0x0E, 0x01, 0x03, 0x01, 0x01, 0x00, 0x00, 0x00,
];

fn parse_ber_len(slice: &[u8]) -> Result<(usize, usize), StanagError> {
    if slice.is_empty() {
        return Err(StanagError::BufferTooShort);
    }
    let first = slice[0];
    if first < 0x80 {
        return Ok((first as usize, 1));
    }
    let n = (first & 0x7F) as usize;
    if n == 0 || slice.len() < 1 + n {
        return Err(StanagError::BufferTooShort);
    }
    let mut len = 0usize;
    for &b in &slice[1..=n] {
        len = (len << 8) | (b as usize);
    }
    Ok((len, 1 + n))
}

fn scan_for_tag2(payload: &[u8]) -> Result<StanagTimestamp, StanagError> {
    let mut idx = 0;
    while idx < payload.len() {
        let tag = payload[idx];
        if idx + 1 >= payload.len() {
            return Err(StanagError::BufferTooShort);
        }
        let (len, len_bytes) = parse_ber_len(&payload[idx + 1..])?;
        let val_start = idx + 1 + len_bytes;
        if payload.len() < val_start + len {
            return Err(StanagError::BufferTooShort);
        }
        if tag == MISB_TAG_PRECISION_TIME_STAMP {
            if len != (MISB_LEN_PRECISION_TIME_STAMP as usize) {
                return Err(StanagError::InvalidKlvLength(len));
            }
            let mut val = [0u8; 8];
            val.copy_from_slice(&payload[val_start..val_start + 8]);
            return Ok(StanagTimestamp::from_be_bytes(val));
        }
        idx = val_start + len;
    }
    Err(StanagError::BufferTooShort)
}

impl StanagTimestamp {
    /// Format standalone MISB ST 0601 KLV packet containing this timestamp.
    pub fn to_klv_packet(&self) -> Vec<u8> {
        let tag2 = self.to_klv_tag2();
        let payload_len = tag2.len() + 4;
        let mut packet = Vec::with_capacity(16 + 2 + payload_len);
        packet.extend_from_slice(&MISB_0601_UNIVERSAL_KEY);
        packet.push(payload_len as u8);
        packet.extend_from_slice(&tag2);
        packet.extend_from_slice(&[0x01, 0x02]);
        let ck = compute_st0601_checksum(&packet);
        packet.extend_from_slice(&ck.to_be_bytes());
        packet
    }

    /// Parse a standalone MISB ST 0601 KLV packet and extract the timestamp.
    pub fn from_klv_packet(packet: &[u8]) -> Result<Self, StanagError> {
        if packet.len() < 22 || packet[..16] != MISB_0601_UNIVERSAL_KEY {
            return Err(StanagError::BufferTooShort);
        }
        let (body_len, len_bytes) = parse_ber_len(&packet[16..])?;
        let offset = 16 + len_bytes;
        if packet.len() < offset + body_len || body_len < 4 {
            return Err(StanagError::BufferTooShort);
        }
        let full_len = offset + body_len;
        let ck_bytes = [packet[full_len - 2], packet[full_len - 1]];
        if compute_st0601_checksum(&packet[..full_len - 2]) != u16::from_be_bytes(ck_bytes) {
            return Err(StanagError::InvalidChecksum);
        }
        scan_for_tag2(&packet[offset..full_len - 4])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_packet_sanity() {
        let ts = StanagTimestamp::from_micros(1_000_000);
        let packet = ts.to_klv_packet();
        assert_eq!(packet.len(), 31);
        assert_eq!(&packet[..16], &MISB_0601_UNIVERSAL_KEY);
    }
}
