//! Maidenhead Locator System (QTH locator) encoding and decoding.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaidenheadError {
    InvalidLength,
    InvalidCharacter,
    LatitudeOutOfRange,
    LongitudeOutOfRange,
    InvalidPrecision,
}

impl std::fmt::Display for MaidenheadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "locator must be 2, 4, or 6 characters"),
            Self::InvalidCharacter => write!(f, "invalid character in locator string"),
            Self::LatitudeOutOfRange => write!(f, "latitude must be within [-90.0, 90.0]"),
            Self::LongitudeOutOfRange => write!(f, "longitude must be within [-180.0, 180.0]"),
            Self::InvalidPrecision => write!(f, "precision must be 1, 2, or 3 pairs"),
        }
    }
}

impl std::error::Error for MaidenheadError {}

fn char_offset(c: u8, min: u8, max: u8) -> Result<f64, MaidenheadError> {
    if (min..=max).contains(&c) {
        Ok((c - min) as f64)
    } else {
        Err(MaidenheadError::InvalidCharacter)
    }
}

/// Decode a Maidenhead locator into center (latitude, longitude) in degrees.
pub fn decode_maidenhead(loc: &str) -> Result<(f64, f64), MaidenheadError> {
    let b = loc.as_bytes();
    if ![2, 4, 6].contains(&b.len()) {
        return Err(MaidenheadError::InvalidLength);
    }
    let lon0 = char_offset(b[0].to_ascii_uppercase(), b'A', b'R')?;
    let lat0 = char_offset(b[1].to_ascii_uppercase(), b'A', b'R')?;
    let mut lon = -170.0 + lon0 * 20.0;
    let mut lat = -85.0 + lat0 * 10.0;
    if b.len() >= 4 {
        lon += char_offset(b[2], b'0', b'9')? * 2.0 - 9.0;
        lat += char_offset(b[3], b'0', b'9')? * 1.0 - 4.5;
    }
    if b.len() == 6 {
        lon += char_offset(b[4].to_ascii_lowercase(), b'a', b'x')? * (1.0 / 12.0) - (23.0 / 24.0);
        lat += char_offset(b[5].to_ascii_lowercase(), b'a', b'x')? * (1.0 / 24.0) - (23.0 / 48.0);
    }
    Ok((lat, lon))
}
