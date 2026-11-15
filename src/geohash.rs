//! Standard base32 geohash encoder and decoder with bounding box calculation.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeohashError {
    InvalidCharacter(char),
    InvalidPrecision,
    EmptyHash,
    LatitudeOutOfRange,
    LongitudeOutOfRange,
}

impl std::fmt::Display for GeohashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCharacter(c) => write!(f, "invalid character in geohash: '{c}'"),
            Self::InvalidPrecision => write!(f, "precision must be between 1 and 12"),
            Self::EmptyHash => write!(f, "geohash string cannot be empty"),
            Self::LatitudeOutOfRange => write!(f, "latitude must be within [-90.0, 90.0]"),
            Self::LongitudeOutOfRange => write!(f, "longitude must be within [-180.0, 180.0]"),
        }
    }
}

impl std::error::Error for GeohashError {}

pub use crate::geo_bbox::GeoBbox;

const BASE32_CHARS: &[u8; 32] = b"0123456789bcdefghjkmnpqrstuvwxyz";

fn char_to_value(c: char) -> Result<u8, GeohashError> {
    let lower = c.to_ascii_lowercase();
    match BASE32_CHARS.iter().position(|&b| b == lower as u8) {
        Some(idx) => Ok(idx as u8),
        None => Err(GeohashError::InvalidCharacter(c)),
    }
}
