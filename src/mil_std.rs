//! MIL-STD-2525 tactical symbol parser and generator.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MilStdError {
    InvalidLength,
    InvalidCharacter,
    InvalidAffiliation,
    InvalidDimension,
    InvalidStatus,
    InvalidEchelon,
}

impl std::fmt::Display for MilStdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "invalid SIDC length"),
            Self::InvalidCharacter => write!(f, "invalid SIDC character"),
            Self::InvalidAffiliation => write!(f, "invalid affiliation"),
            Self::InvalidDimension => write!(f, "invalid dimension"),
            Self::InvalidStatus => write!(f, "invalid status"),
            Self::InvalidEchelon => write!(f, "invalid echelon"),
        }
    }
}

impl std::error::Error for MilStdError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MilStdAffiliation {
    #[default]
    Unknown,
    Pending,
    Friend,
    AssumedFriend,
    Neutral,
    Hostile,
    Suspect,
    Joker,
    Faker,
}
