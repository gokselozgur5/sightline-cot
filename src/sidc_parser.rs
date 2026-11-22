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
    Joker = 8,
    Faker = 9,
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
            8 => Ok(Self::Joker),
            9 => Ok(Self::Faker),
            _ => Err(SidcParseError::InvalidAffiliation),
        }
    }
    pub fn code(&self) -> u8 {
        *self as u8
    }

    pub fn cot_char(&self) -> char {
        match self {
            Self::Friend | Self::AssumedFriend => 'f',
            Self::Hostile | Self::Joker | Self::Faker => 'h',
            Self::Neutral => 'n',
            Self::Suspect => 's',
            Self::Pending => 'p',
            Self::Unknown => 'u',
        }
    }

    pub fn from_cot_char(c: char) -> Self {
        match c {
            'a' | 'A' => Self::AssumedFriend,
            'f' | 'F' => Self::Friend,
            'h' | 'H' | 'j' | 'J' | 'k' | 'K' => Self::Hostile,
            'n' | 'N' => Self::Neutral,
            's' | 'S' => Self::Suspect,
            'p' | 'P' => Self::Pending,
            _ => Self::Unknown,
        }
    }

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

    pub fn from_2525c_char(c: char) -> Result<Self, SidcParseError> {
        match c {
            'P' => Ok(Self::Pending),
            'U' => Ok(Self::Unknown),
            'A' => Ok(Self::AssumedFriend),
            'F' => Ok(Self::Friend),
            'N' => Ok(Self::Neutral),
            'S' => Ok(Self::Suspect),
            'H' => Ok(Self::Hostile),
            'J' => Ok(Self::Joker),
            'K' => Ok(Self::Faker),
            _ => Err(SidcParseError::InvalidAffiliation),
        }
    }

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

    pub fn from_2525d_code(code: &str) -> Result<Self, SidcParseError> {
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
            _ => Err(SidcParseError::InvalidAffiliation),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SymbolSet {
    Unknown = 0,
    Air = 1,
    AirMissile = 2,
    Space = 5,
    #[default]
    LandUnit = 10,
    LandCivilian = 11,
    LandEquipment = 15,
    LandInstallation = 20,
    SeaSurface = 30,
    Subsurface = 35,
    Activity = 40,
    Cyberspace = 60,
}

impl SymbolSet {
    pub fn from_code(val: u8) -> Result<Self, SidcParseError> {
        match val {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Air),
            2 => Ok(Self::AirMissile),
            5 => Ok(Self::Space),
            10 => Ok(Self::LandUnit),
            11 => Ok(Self::LandCivilian),
            15 => Ok(Self::LandEquipment),
            20 => Ok(Self::LandInstallation),
            30 => Ok(Self::SeaSurface),
            35 => Ok(Self::Subsurface),
            40 => Ok(Self::Activity),
            60 => Ok(Self::Cyberspace),
            _ => Err(SidcParseError::InvalidSymbolSet),
        }
    }

    pub fn code(&self) -> u8 {
        *self as u8
    }
    pub fn cot_dimension_str(&self) -> &'static str {
        match self {
            Self::Air | Self::AirMissile => "A",
            Self::Space => "P",
            Self::SeaSurface => "S",
            Self::Subsurface => "U",
            Self::Activity => "t",
            Self::Cyberspace => "X",
            _ => "G",
        }
    }

    pub fn from_cot_dimension(dim: &str) -> Self {
        match dim {
            "A" => Self::Air,
            "P" => Self::Space,
            "S" => Self::SeaSurface,
            "U" => Self::Subsurface,
            "t" | "T" => Self::Activity,
            "X" | "x" => Self::Cyberspace,
            _ => Self::LandUnit,
        }
    }

    pub fn to_2525c_char(&self) -> char {
        match self {
            Self::Air | Self::AirMissile => 'A',
            Self::Space => 'P',
            Self::SeaSurface => 'S',
            Self::Subsurface => 'U',
            Self::Activity | Self::Cyberspace => 'X',
            _ => 'G',
        }
    }

    pub fn from_2525c_char(c: char) -> Result<Self, SidcParseError> {
        match c {
            'G' => Ok(Self::LandUnit),
            'A' => Ok(Self::Air),
            'S' => Ok(Self::SeaSurface),
            'U' => Ok(Self::Subsurface),
            'P' => Ok(Self::Space),
            'X' => Ok(Self::Activity),
            _ => Err(SidcParseError::InvalidSymbolSet),
        }
    }

    pub fn to_2525d_code(&self) -> &'static str {
        match self {
            Self::Air => "01",
            Self::AirMissile => "02",
            Self::Space => "05",
            Self::LandUnit => "10",
            Self::LandCivilian => "11",
            Self::LandEquipment => "15",
            Self::LandInstallation => "20",
            Self::SeaSurface => "30",
            Self::Subsurface => "35",
            Self::Activity => "40",
            Self::Cyberspace => "60",
            Self::Unknown => "00",
        }
    }

    pub fn from_2525d_code(code: &str) -> Result<Self, SidcParseError> {
        match code {
            "00" => Ok(Self::Unknown),
            "01" => Ok(Self::Air),
            "02" => Ok(Self::AirMissile),
            "05" => Ok(Self::Space),
            "10" => Ok(Self::LandUnit),
            "11" => Ok(Self::LandCivilian),
            "15" => Ok(Self::LandEquipment),
            "20" => Ok(Self::LandInstallation),
            "30" => Ok(Self::SeaSurface),
            "35" => Ok(Self::Subsurface),
            "40" => Ok(Self::Activity),
            "60" => Ok(Self::Cyberspace),
            _ => Err(SidcParseError::InvalidSymbolSet),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SymbolStatus {
    #[default]
    Present = 0,
    Planned = 1,
    PresentFullyCapable = 2,
    PresentDamaged = 3,
    PresentDestroyed = 4,
    PresentFullToCapacity = 5,
}

impl SymbolStatus {
    pub fn from_code(val: u8) -> Result<Self, SidcParseError> {
        match val {
            0 => Ok(Self::Present),
            1 => Ok(Self::Planned),
            2 => Ok(Self::PresentFullyCapable),
            3 => Ok(Self::PresentDamaged),
            4 => Ok(Self::PresentDestroyed),
            5 => Ok(Self::PresentFullToCapacity),
            _ => Err(SidcParseError::InvalidStatus),
        }
    }

    pub fn code(&self) -> u8 {
        *self as u8
    }

    pub fn from_char(c: char) -> Result<Self, SidcParseError> {
        match c {
            '0' | 'P' => Ok(Self::Present),
            '1' | 'A' => Ok(Self::Planned),
            '2' => Ok(Self::PresentFullyCapable),
            '3' => Ok(Self::PresentDamaged),
            '4' => Ok(Self::PresentDestroyed),
            '5' => Ok(Self::PresentFullToCapacity),
            _ => Err(SidcParseError::InvalidStatus),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Present => 'P',
            Self::Planned => 'A',
            _ => 'P',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Echelon {
    #[default]
    Unspecified = 0,
    Team = 11,
    Squad = 12,
    Section = 13,
    Platoon = 14,
    Company = 15,
    Battalion = 16,
    Regiment = 17,
    Brigade = 18,
    Division = 21,
    Corps = 22,
    Army = 23,
    ArmyGroup = 24,
    Region = 25,
    Command = 26,
    WheeledLimited = 31,
    WheeledCrossCountry = 32,
    Tracked = 33,
    WheeledTrackedCombination = 34,
    Towed = 35,
    Rail = 36,
    PackAnimals = 37,
    OverSnow = 41,
    Sled = 42,
    Amphibious = 51,
    TowedArray = 52,
}

impl Echelon {
    pub fn from_code(val: u8) -> Result<Self, SidcParseError> {
        match val {
            0 => Ok(Self::Unspecified),
            11 => Ok(Self::Team),
            12 => Ok(Self::Squad),
            13 => Ok(Self::Section),
            14 => Ok(Self::Platoon),
            15 => Ok(Self::Company),
            16 => Ok(Self::Battalion),
            17 => Ok(Self::Regiment),
            18 => Ok(Self::Brigade),
            21 => Ok(Self::Division),
            22 => Ok(Self::Corps),
            23 => Ok(Self::Army),
            24 => Ok(Self::ArmyGroup),
            25 => Ok(Self::Region),
            26 => Ok(Self::Command),
            31 => Ok(Self::WheeledLimited),
            32 => Ok(Self::WheeledCrossCountry),
            33 => Ok(Self::Tracked),
            34 => Ok(Self::WheeledTrackedCombination),
            35 => Ok(Self::Towed),
            36 => Ok(Self::Rail),
            37 => Ok(Self::PackAnimals),
            41 => Ok(Self::OverSnow),
            42 => Ok(Self::Sled),
            51 => Ok(Self::Amphibious),
            52 => Ok(Self::TowedArray),
            _ => Err(SidcParseError::InvalidEchelon),
        }
    }

    pub fn code(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilStd2525DSymbol {
    pub version: u8,
    pub context: u8,
    pub identity: StandardIdentity,
    pub symbol_set: SymbolSet,
    pub status: SymbolStatus,
    pub hq_tf_dummy: u8,
    pub echelon: Echelon,
    pub entity_code: u32,
    pub modifier1: u8,
    pub modifier2: u8,
}

impl Default for MilStd2525DSymbol {
    fn default() -> Self {
        Self {
            version: 10,
            context: 0,
            identity: StandardIdentity::Friend,
            symbol_set: SymbolSet::LandUnit,
            status: SymbolStatus::Present,
            hq_tf_dummy: 0,
            echelon: Echelon::Unspecified,
            entity_code: 0,
            modifier1: 0,
            modifier2: 0,
        }
    }
}

impl MilStd2525DSymbol {
    pub fn parse(s: &str) -> Result<Self, SidcParseError> {
        let clean = s.trim();
        if clean.len() != 20 && clean.len() != 30 {
            return Err(SidcParseError::InvalidLength);
        }
        if !clean.chars().all(|c| c.is_ascii_digit()) {
            return Err(SidcParseError::NonNumeric);
        }
        let ver: u8 = clean[0..2]
            .parse()
            .map_err(|_| SidcParseError::InvalidStandard)?;
        if ver != 10 && ver != 11 && ver != 30 {
            return Err(SidcParseError::InvalidStandard);
        }
        let context: u8 = clean[2..3]
            .parse()
            .map_err(|_| SidcParseError::NonNumeric)?;
        let id_val: u8 = clean[3..4]
            .parse()
            .map_err(|_| SidcParseError::NonNumeric)?;
        let identity = StandardIdentity::from_code(id_val)?;
        let set_val: u8 = clean[4..6]
            .parse()
            .map_err(|_| SidcParseError::NonNumeric)?;
        let symbol_set = SymbolSet::from_code(set_val)?;
        let stat_val: u8 = clean[6..7]
            .parse()
            .map_err(|_| SidcParseError::NonNumeric)?;
        let status = SymbolStatus::from_code(stat_val)?;
        let hq_tf_dummy: u8 = clean[7..8]
            .parse()
            .map_err(|_| SidcParseError::NonNumeric)?;
        let ech_val: u8 = clean[8..10]
            .parse()
            .map_err(|_| SidcParseError::NonNumeric)?;
        let echelon = Echelon::from_code(ech_val)?;
        let entity_code: u32 = clean[10..16]
            .parse()
            .map_err(|_| SidcParseError::NonNumeric)?;
        let modifier1: u8 = clean[16..18]
            .parse()
            .map_err(|_| SidcParseError::NonNumeric)?;
        let modifier2: u8 = clean[18..20]
            .parse()
            .map_err(|_| SidcParseError::NonNumeric)?;

        Ok(Self {
            version: ver,
            context,
            identity,
            symbol_set,
            status,
            hq_tf_dummy,
            echelon,
            entity_code,
            modifier1,
            modifier2,
        })
    }
    pub fn to_sidc20(&self) -> String {
        format!(
            "{:02}{:01}{:01}{:02}{:01}{:01}{:02}{:06}{:02}{:02}",
            self.version,
            self.context % 10,
            self.identity.code() % 10,
            self.symbol_set.code(),
            self.status.code() % 10,
            self.hq_tf_dummy % 10,
            self.echelon.code(),
            self.entity_code % 1_000_000,
            self.modifier1 % 100,
            self.modifier2 % 100
        )
    }
    pub fn to_cot_type(&self) -> String {
        let aff = self.identity.cot_char();
        let dim = self.symbol_set.cot_dimension_str();
        match self.symbol_set {
            SymbolSet::LandEquipment => format!("a-{aff}-{dim}-E"),
            SymbolSet::LandInstallation => format!("a-{aff}-{dim}-I"),
            SymbolSet::AirMissile => format!("a-{aff}-{dim}-M"),
            _ => format!("a-{aff}-{dim}"),
        }
    }
    pub fn from_cot_type(cot: &str) -> Result<Self, SidcParseError> {
        let trimmed = cot.trim();
        let parts: Vec<&str> = trimmed.split('-').collect();
        if parts.is_empty() || parts[0] != "a" {
            return Err(SidcParseError::InvalidCotType);
        }
        let identity = if parts.len() > 1 && !parts[1].is_empty() {
            StandardIdentity::from_cot_char(parts[1].chars().next().unwrap())
        } else {
            StandardIdentity::Unknown
        };
        let dim_str = if parts.len() > 2 { parts[2] } else { "G" };
        let mut symbol_set = SymbolSet::from_cot_dimension(dim_str);
        if parts.len() > 3 {
            if dim_str == "G" && parts[3] == "E" {
                symbol_set = SymbolSet::LandEquipment;
            } else if dim_str == "G" && parts[3] == "I" {
                symbol_set = SymbolSet::LandInstallation;
            } else if dim_str == "A" && parts[3] == "M" {
                symbol_set = SymbolSet::AirMissile;
            }
        }
        Ok(Self {
            version: 10,
            context: 0,
            identity,
            symbol_set,
            status: SymbolStatus::Present,
            hq_tf_dummy: 0,
            echelon: Echelon::Unspecified,
            entity_code: 0,
            modifier1: 0,
            modifier2: 0,
        })
    }
}

pub fn parse_sidc2525d(s: &str) -> Result<MilStd2525DSymbol, SidcParseError> {
    MilStd2525DSymbol::parse(s)
}

pub fn generate_sidc2525d(sym: &MilStd2525DSymbol) -> String {
    sym.to_sidc20()
}

pub fn cot_type_to_sidc20(cot: &str) -> Result<String, SidcParseError> {
    let sym = MilStd2525DSymbol::from_cot_type(cot)?;
    Ok(sym.to_sidc20())
}

pub fn sidc20_to_cot_type(sidc: &str) -> Result<String, SidcParseError> {
    let sym = MilStd2525DSymbol::parse(sidc)?;
    Ok(sym.to_cot_type())
}
