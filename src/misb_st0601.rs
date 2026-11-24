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

