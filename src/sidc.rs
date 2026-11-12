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

impl Default for TacticalSymbol {
    fn default() -> Self {
        Self {
            version: SidcVersion::MilStd2525D,
            affiliation: SidcAffiliation::Unknown,
            domain: SidcDomain::Land,
            entity_code: 120100,
            modifier: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SidcBuffer {
    bytes: [u8; 32],
    len: usize,
}

impl SidcBuffer {
    pub const fn empty() -> Self {
        Self {
            bytes: [0u8; 32],
            len: 0,
        }
    }
}

impl SidcBuffer {
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("")
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl SidcAffiliation {
    pub fn to_2525d_code(&self) -> &'static str {
        match self {
            Self::Pending => "01",
            Self::Unknown => "02",
            Self::AssumedFriend => "04",
            Self::Friend => "03",
            Self::Neutral => "05",
            Self::Suspect => "07",
            Self::Hostile => "06",
            Self::Joker => "08",
            Self::Faker => "09",
        }
    }
}

impl SidcAffiliation {
    pub fn from_2525d_code(code: &str) -> Result<Self, SidcError> {
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
            _ => Err(SidcError::InvalidAffiliation),
        }
    }
}

impl SidcAffiliation {
    pub fn to_2525c_char(&self) -> char {
        match self {
            Self::Pending => 'P',
            Self::Unknown => 'U',
            Self::AssumedFriend => 'A',
            Self::Friend => 'F',
            Self::Neutral => 'N',
            Self::Suspect => 'S',
            Self::Hostile => 'H',
            Self::Joker => 'J',
            Self::Faker => 'K',
        }
    }
}

impl SidcAffiliation {
    pub fn from_2525c_char(c: char) -> Result<Self, SidcError> {
        match c {
            'P' => Ok(Self::Pending),
            'U' => Ok(Self::Unknown),
            'F' => Ok(Self::Friend),
            'N' => Ok(Self::Neutral),
            'H' => Ok(Self::Hostile),
            'A' => Ok(Self::AssumedFriend),
            'S' => Ok(Self::Suspect),
            'J' => Ok(Self::Joker),
            'K' => Ok(Self::Faker),
            _ => Err(SidcError::InvalidAffiliation),
        }
    }
}

impl SidcDomain {
    pub fn to_2525d_code(&self) -> &'static str {
        match self {
            Self::Air => "01",
            Self::Space => "05",
            Self::Land => "10",
            Self::Surface => "30",
            Self::Subsurface => "35",
            Self::Activity => "40",
        }
    }
}

impl SidcDomain {
    pub fn from_2525d_code(code: &str) -> Result<Self, SidcError> {
        match code {
            "01" => Ok(Self::Air),
            "05" => Ok(Self::Space),
            "10" => Ok(Self::Land),
            "30" => Ok(Self::Surface),
            "35" => Ok(Self::Subsurface),
            "40" => Ok(Self::Activity),
            _ => Err(SidcError::InvalidDomain),
        }
    }
}

impl SidcDomain {
    pub fn to_2525c_char(&self) -> char {
        match self {
            Self::Air => 'A',
            Self::Space => 'P',
            Self::Land => 'G',
            Self::Surface => 'S',
            Self::Subsurface => 'U',
            Self::Activity => 'X',
        }
    }
}
