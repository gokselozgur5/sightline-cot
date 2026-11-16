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

impl MilStdAffiliation {
    pub fn to_2525c_char(&self) -> char {
        match self {
            Self::Pending => 'P',
            Self::Unknown => 'U',
            Self::Friend => 'F',
            Self::AssumedFriend => 'A',
            Self::Neutral => 'N',
            Self::Hostile => 'H',
            Self::Suspect => 'S',
            Self::Joker => 'J',
            Self::Faker => 'K',
        }
    }
}

impl MilStdAffiliation {
    pub fn from_2525c_char(c: char) -> Result<Self, MilStdError> {
        match c {
            'P' => Ok(Self::Pending),
            'U' => Ok(Self::Unknown),
            'F' => Ok(Self::Friend),
            'A' => Ok(Self::AssumedFriend),
            'N' => Ok(Self::Neutral),
            'H' => Ok(Self::Hostile),
            'S' => Ok(Self::Suspect),
            'J' => Ok(Self::Joker),
            'K' => Ok(Self::Faker),
            _ => Err(MilStdError::InvalidAffiliation),
        }
    }
}

impl MilStdAffiliation {
    pub fn from_2525d_code(code: &str) -> Result<Self, MilStdError> {
        match code {
            "01" => Ok(Self::Pending),
            "02" => Ok(Self::Unknown),
            "03" => Ok(Self::Friend),
            "04" => Ok(Self::AssumedFriend),
            "05" => Ok(Self::Neutral),
            "06" => Ok(Self::Hostile),
            "07" => Ok(Self::Suspect),
            "08" => Ok(Self::Joker),
            "09" => Ok(Self::Faker),
            _ => Err(MilStdError::InvalidAffiliation),
        }
    }
}

impl MilStdAffiliation {
    pub fn to_2525d_code(&self) -> &'static str {
        match self {
            Self::Pending => "01",
            Self::Unknown => "02",
            Self::Friend => "03",
            Self::AssumedFriend => "04",
            Self::Neutral => "05",
            Self::Hostile => "06",
            Self::Suspect => "07",
            Self::Joker => "08",
            Self::Faker => "09",
        }
    }
}
