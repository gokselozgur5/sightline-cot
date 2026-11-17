//! MIL-STD-2525D tactical symbol SIDC 20-digit parser and generator.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidcParseError {
    InvalidLength,
    NonNumeric,
    InvalidStandard,
    InvalidAffiliation,
    InvalidSymbolSet,
    InvalidStatus,
    InvalidEchelon,
    InvalidCotType,
}

impl core::fmt::Display for SidcParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "SIDC must be 20 digits"),
            Self::NonNumeric => write!(f, "SIDC must contain only digits"),
            Self::InvalidStandard => write!(f, "unsupported version code"),
            Self::InvalidAffiliation => write!(f, "invalid affiliation code"),
            Self::InvalidSymbolSet => write!(f, "invalid symbol set code"),
            Self::InvalidStatus => write!(f, "invalid status code"),
            Self::InvalidEchelon => write!(f, "invalid echelon code"),
            Self::InvalidCotType => write!(f, "invalid CoT type string"),
        }
    }
}

impl std::error::Error for SidcParseError {}
