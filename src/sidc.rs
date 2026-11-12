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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidcAffiliation {
    Pending,
    #[default]
    Unknown,
    Friend,
    Neutral,
    Hostile,
    AssumedFriend,
    Suspect,
    Joker,
    Faker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidcDomain {
    #[default]
    Land,
    Air,
    Surface,
    Subsurface,
    Space,
    Activity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidcVersion {
    Legacy2525C,
    #[default]
    MilStd2525D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TacticalSymbol {
    pub version: SidcVersion,
    pub affiliation: SidcAffiliation,
    pub domain: SidcDomain,
    pub entity_code: u32,
    pub modifier: u8,
}
