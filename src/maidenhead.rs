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
