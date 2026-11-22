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
            "00" => Ok(Self::Pending),
            "01" => Ok(Self::Unknown),
            "02" => Ok(Self::AssumedFriend),
            "03" => Ok(Self::Friend),
            "04" => Ok(Self::Neutral),
            "05" => Ok(Self::Suspect),
            "06" => Ok(Self::Hostile),
            "08" => Ok(Self::Joker),
            "09" => Ok(Self::Faker),
            _ => Err(MilStdError::InvalidAffiliation),
        }
    }
}

impl MilStdAffiliation {
    pub fn to_2525d_code(&self) -> &'static str {
        match self {
            Self::Pending => "00",
            Self::Unknown => "01",
            Self::AssumedFriend => "02",
            Self::Friend => "03",
            Self::Neutral => "04",
            Self::Suspect => "05",
            Self::Hostile => "06",
            Self::Joker => "08",
            Self::Faker => "09",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MilStdDimension {
    #[default]
    Land,
    Air,
    Surface,
    Subsurface,
    Space,
    Activity,
}

impl MilStdDimension {
    pub fn to_2525c_char(&self) -> char {
        match self {
            Self::Land => 'G',
            Self::Air => 'A',
            Self::Surface => 'S',
            Self::Subsurface => 'U',
            Self::Space => 'P',
            Self::Activity => 'X',
        }
    }
}

impl MilStdDimension {
    pub fn from_2525c_char(c: char) -> Result<Self, MilStdError> {
        match c {
            'G' => Ok(Self::Land),
            'A' => Ok(Self::Air),
            'S' => Ok(Self::Surface),
            'U' => Ok(Self::Subsurface),
            'P' => Ok(Self::Space),
            'X' => Ok(Self::Activity),
            _ => Err(MilStdError::InvalidDimension),
        }
    }
}

impl MilStdDimension {
    pub fn from_2525d_code(code: &str) -> Result<Self, MilStdError> {
        match code {
            "01" => Ok(Self::Air),
            "05" => Ok(Self::Space),
            "10" => Ok(Self::Land),
            "30" => Ok(Self::Surface),
            "35" => Ok(Self::Subsurface),
            "40" => Ok(Self::Activity),
            _ => Err(MilStdError::InvalidDimension),
        }
    }
}

impl MilStdDimension {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MilStdStatus {
    #[default]
    Present,
    Planned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MilStdEchelon {
    #[default]
    Unspecified,
    Team,
    Squad,
    Section,
    Platoon,
    Company,
    Battalion,
    Regiment,
    Brigade,
    Division,
    Corps,
    Army,
}

impl MilStdStatus {
    pub fn from_char(c: char) -> Result<Self, MilStdError> {
        match c {
            'P' | '0' => Ok(Self::Present),
            'A' | '1' => Ok(Self::Planned),
            _ => Err(MilStdError::InvalidStatus),
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            Self::Present => 'P',
            Self::Planned => 'A',
        }
    }
}

impl MilStdEchelon {
    pub fn from_2525d_code(code: &str) -> Result<Self, MilStdError> {
        match code {
            "00" => Ok(Self::Unspecified),
            "11" => Ok(Self::Team),
            "12" => Ok(Self::Squad),
            "13" => Ok(Self::Section),
            "14" => Ok(Self::Platoon),
            "15" => Ok(Self::Company),
            "16" => Ok(Self::Battalion),
            "17" => Ok(Self::Regiment),
            "18" => Ok(Self::Brigade),
            "21" => Ok(Self::Division),
            "22" => Ok(Self::Corps),
            "23" => Ok(Self::Army),
            _ => Err(MilStdError::InvalidEchelon),
        }
    }
}

impl MilStdEchelon {
    pub fn to_2525d_code(&self) -> &'static str {
        match self {
            Self::Unspecified => "00",
            Self::Team => "11",
            Self::Squad => "12",
            Self::Section => "13",
            Self::Platoon => "14",
            Self::Company => "15",
            Self::Battalion => "16",
            Self::Regiment => "17",
            Self::Brigade => "18",
            Self::Division => "21",
            Self::Corps => "22",
            Self::Army => "23",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilStdSymbol {
    pub affiliation: MilStdAffiliation,
    pub dimension: MilStdDimension,
    pub status: MilStdStatus,
    pub echelon: MilStdEchelon,
    pub function_id: [char; 6],
    pub country_code: [char; 2],
    pub modifier: [char; 2],
    pub order_of_battle: char,
}

impl Default for MilStdSymbol {
    fn default() -> Self {
        Self {
            affiliation: MilStdAffiliation::Unknown,
            dimension: MilStdDimension::Land,
            status: MilStdStatus::Present,
            echelon: MilStdEchelon::Unspecified,
            function_id: ['-', '-', '-', '-', '-', '-'],
            country_code: ['-', '-'],
            modifier: ['-', '-'],
            order_of_battle: '-',
        }
    }
}

impl MilStdSymbol {
    pub fn parse_2525c(s: &str) -> Result<Self, MilStdError> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 15 {
            return Err(MilStdError::InvalidLength);
        }
        let aff = MilStdAffiliation::from_2525c_char(chars[1])?;
        let dim = MilStdDimension::from_2525c_char(chars[2])?;
        let status = MilStdStatus::from_char(chars[3])?;
        let mut fid = ['-'; 6];
        for (i, c) in chars[4..10].iter().enumerate() {
            fid[i] = *c;
        }
        Ok(Self {
            affiliation: aff,
            dimension: dim,
            status,
            function_id: fid,
            modifier: [chars[10], chars[11]],
            country_code: [chars[12], chars[13]],
            order_of_battle: chars[14],
            ..Default::default()
        })
    }
}

impl MilStdSymbol {
    pub fn parse_2525d(s: &str) -> Result<Self, MilStdError> {
        if s.len() < 20 || s.len() > 30 {
            return Err(MilStdError::InvalidLength);
        }
        if !s.chars().all(|c| c.is_ascii_digit()) {
            return Err(MilStdError::InvalidCharacter);
        }
        let aff = MilStdAffiliation::from_2525d_code(&s[2..4])?;
        let dim = MilStdDimension::from_2525d_code(&s[4..6])?;
        let status_ch = s.chars().nth(6).unwrap_or('0');
        let status = MilStdStatus::from_char(status_ch)?;
        let echelon = MilStdEchelon::from_2525d_code(&s[8..10])?;
        Ok(Self {
            affiliation: aff,
            dimension: dim,
            status,
            echelon,
            ..Default::default()
        })
    }
}

pub fn parse_mil_std(sidc: &str) -> Result<MilStdSymbol, MilStdError> {
    let trimmed = sidc.trim();
    if !trimmed.is_ascii() {
        return Err(MilStdError::InvalidCharacter);
    }
    if trimmed.len() == 15 {
        MilStdSymbol::parse_2525c(trimmed)
    } else if trimmed.len() >= 20 && trimmed.len() <= 30 {
        MilStdSymbol::parse_2525d(trimmed)
    } else {
        Err(MilStdError::InvalidLength)
    }
}

impl MilStdSymbol {
    pub fn to_2525c_string(&self) -> String {
        let mut s = String::with_capacity(15);
        s.push('S');
        s.push(self.affiliation.to_2525c_char());
        s.push(self.dimension.to_2525c_char());
        s.push(self.status.to_char());
        for &c in &self.function_id {
            s.push(c);
        }
        s.push(self.modifier[0]);
        s.push(self.modifier[1]);
        s.push(self.country_code[0]);
        s.push(self.country_code[1]);
        s.push(self.order_of_battle);
        s
    }
}

impl MilStdSymbol {
    pub fn to_2525d_string(&self) -> String {
        let mut s = String::with_capacity(30);
        s.push_str("10");
        s.push_str(self.affiliation.to_2525d_code());
        s.push_str(self.dimension.to_2525d_code());
        s.push(if self.status == MilStdStatus::Present {
            '0'
        } else {
            '1'
        });
        s.push('0');
        s.push_str(self.echelon.to_2525d_code());
        s.push_str("0000000000");
        s
    }
}

impl MilStdSymbol {
    pub fn to_cot_type(&self) -> String {
        let aff = match self.affiliation {
            MilStdAffiliation::Friend | MilStdAffiliation::AssumedFriend => "f",
            MilStdAffiliation::Hostile | MilStdAffiliation::Joker | MilStdAffiliation::Faker => "h",
            MilStdAffiliation::Neutral => "n",
            MilStdAffiliation::Suspect => "s",
            MilStdAffiliation::Pending => "p",
            _ => "u",
        };
        let dim = match self.dimension {
            MilStdDimension::Air => "A",
            MilStdDimension::Surface => "S",
            MilStdDimension::Subsurface => "U",
            MilStdDimension::Space => "P",
            _ => "G",
        };
        format!("a-{aff}-{dim}")
    }
}
