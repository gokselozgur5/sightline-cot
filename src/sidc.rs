//! MIL-STD-2525D and APP-6 tactical Symbol Identification Code (SIDC).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidcError {
    InvalidLength,
    InvalidCharacter,
    UnknownStandard,
    InvalidAffiliation,
    InvalidDomain,
}

impl core::fmt::Display for SidcError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "invalid SIDC length"),
            Self::InvalidCharacter => write!(f, "invalid character in SIDC"),
            Self::UnknownStandard => write!(f, "unrecognized SIDC standard"),
            Self::InvalidAffiliation => write!(f, "invalid identity/affiliation"),
            Self::InvalidDomain => write!(f, "invalid battle dimension or domain"),
        }
    }
}
