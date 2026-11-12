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

impl SidcDomain {
    pub fn from_2525c_char(c: char) -> Result<Self, SidcError> {
        match c {
            'A' => Ok(Self::Air),
            'P' => Ok(Self::Space),
            'G' => Ok(Self::Land),
            'S' => Ok(Self::Surface),
            'U' => Ok(Self::Subsurface),
            'X' => Ok(Self::Activity),
            _ => Err(SidcError::InvalidDomain),
        }
    }
}

impl TacticalSymbol {
    pub fn to_2525d(&self) -> SidcBuffer {
        let mut buf = [b'0'; 32];
        buf[0] = b'1';
        buf[1] = b'0';
        let aff = self.affiliation.to_2525d_code().as_bytes();
        buf[2] = aff[0];
        buf[3] = aff[1];
        let dom = self.domain.to_2525d_code().as_bytes();
        buf[4] = dom[0];
        buf[5] = dom[1];
        SidcBuffer {
            bytes: buf,
            len: 30,
        }
    }
}

impl TacticalSymbol {
    pub fn to_2525c(&self) -> SidcBuffer {
        let mut buf = [b'-'; 32];
        buf[0] = b'S';
        buf[1] = self.affiliation.to_2525c_char() as u8;
        buf[2] = self.domain.to_2525c_char() as u8;
        buf[3] = b'P';
        SidcBuffer {
            bytes: buf,
            len: 15,
        }
    }
}

impl TacticalSymbol {
    pub fn from_sidc(sidc: &str) -> Result<Self, SidcError> {
        let trimmed = sidc.trim();
        if trimmed.len() == 30 {
            Self::parse_2525d(trimmed)
        } else if trimmed.len() == 15 {
            Self::parse_2525c(trimmed)
        } else {
            Err(SidcError::InvalidLength)
        }
    }
}

impl TacticalSymbol {
    fn parse_2525d(s: &str) -> Result<Self, SidcError> {
        let aff = SidcAffiliation::from_2525d_code(&s[2..4])?;
        let dom = SidcDomain::from_2525d_code(&s[4..6])?;
        Ok(Self {
            version: SidcVersion::MilStd2525D,
            affiliation: aff,
            domain: dom,
            entity_code: 0,
            modifier: 0,
        })
    }
}

impl TacticalSymbol {
    fn parse_2525c(s: &str) -> Result<Self, SidcError> {
        let b = s.as_bytes();
        let aff = SidcAffiliation::from_2525c_char(b[1] as char)?;
        let dom = SidcDomain::from_2525c_char(b[2] as char)?;
        Ok(Self {
            version: SidcVersion::Legacy2525C,
            affiliation: aff,
            domain: dom,
            entity_code: 0,
            modifier: 0,
        })
    }
}

fn parse_cot_aff(token: Option<&str>) -> SidcAffiliation {
    match token {
        Some("f") => SidcAffiliation::Friend,
        Some("h") => SidcAffiliation::Hostile,
        Some("n") => SidcAffiliation::Neutral,
        _ => SidcAffiliation::Unknown,
    }
}

fn parse_cot_dom(token: Option<&str>) -> SidcDomain {
    match token {
        Some("A") => SidcDomain::Air,
        Some("S") => SidcDomain::Surface,
        Some("U") => SidcDomain::Subsurface,
        _ => SidcDomain::Land,
    }
}

impl TacticalSymbol {
    pub fn from_cot_type(cot: &str) -> Self {
        let mut it = cot.split('-');
        let _ = it.next();
        let aff = parse_cot_aff(it.next());
        let dom = parse_cot_dom(it.next());
        Self {
            version: SidcVersion::MilStd2525D,
            affiliation: aff,
            domain: dom,
            entity_code: 0,
            modifier: 0,
        }
    }
}
