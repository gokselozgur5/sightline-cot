//! NATO STANAG 4609 / MISB ST 0601 KLV metadata timestamp translator.

/// Errors occurring during STANAG 4609 timestamp parsing and translation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StanagError {
    NegativeTimestamp,
    InvalidKlvTag(u8),
    InvalidKlvLength(usize),
    BufferTooShort,
    InvalidIso8601(String),
}

impl std::fmt::Display for StanagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NegativeTimestamp => write!(f, "timestamp cannot be negative"),
            Self::InvalidKlvTag(t) => write!(f, "invalid KLV tag: 0x{t:02X}, expected Tag 2 (0x02)"),
            Self::InvalidKlvLength(len) => write!(f, "invalid KLV length: {len}, expected 8"),
            Self::BufferTooShort => write!(f, "buffer is too short for KLV timestamp"),
            Self::InvalidIso8601(s) => write!(f, "invalid ISO 8601 date string: {s}"),
        }
    }
}

impl std::error::Error for StanagError {}
