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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StandardIdentity {
    Pending = 0,
    #[default]
    Unknown = 1,
    AssumedFriend = 2,
    Friend = 3,
    Neutral = 4,
    Suspect = 5,
    Hostile = 6,
}

impl StandardIdentity {
    pub fn from_code(val: u8) -> Result<Self, SidcParseError> {
        match val {
            0 => Ok(Self::Pending),
            1 => Ok(Self::Unknown),
            2 => Ok(Self::AssumedFriend),
            3 => Ok(Self::Friend),
            4 => Ok(Self::Neutral),
            5 => Ok(Self::Suspect),
            6 => Ok(Self::Hostile),
            _ => Err(SidcParseError::InvalidAffiliation),
        }
    }
    pub fn code(&self) -> u8 {
        *self as u8
    }

    pub fn cot_char(&self) -> char {
        match self {
            Self::Friend | Self::AssumedFriend => 'f',
            Self::Hostile => 'h',
            Self::Neutral => 'n',
            Self::Suspect => 's',
            Self::Pending => 'p',
            Self::Unknown => 'u',
        }
    }

    pub fn from_cot_char(c: char) -> Self {
        match c {
            'f' | 'F' | 'a' | 'A' => Self::Friend,
            'h' | 'H' | 'j' | 'J' | 'k' | 'K' => Self::Hostile,
            'n' | 'N' => Self::Neutral,
            's' | 'S' => Self::Suspect,
            'p' | 'P' => Self::Pending,
            _ => Self::Unknown,
        }
    }
}
