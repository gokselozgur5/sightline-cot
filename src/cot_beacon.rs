//! CoT emergency beacon 911 and Mayday packet generator.

#[derive(Debug, Clone, PartialEq)]
pub enum CotBeaconError {
    InvalidCoordinate(&'static str),
    MissingField(&'static str),
    InvalidTimestamp(&'static str),
    ParseError(String),
}

impl std::fmt::Display for CotBeaconError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCoordinate(m) => write!(f, "invalid coordinate: {m}"),
            Self::MissingField(m) => write!(f, "missing field: {m}"),
            Self::InvalidTimestamp(m) => write!(f, "invalid timestamp: {m}"),
            Self::ParseError(m) => write!(f, "beacon parse error: {m}"),
        }
    }
}

impl std::error::Error for CotBeaconError {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EmergencyType {
    #[default]
    Alert911,
    Mayday,
    RingTheBell,
    TroopsInContact,
    GeoFenceBreach,
    Cancel,
    Custom(String),
}

impl EmergencyType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Alert911 => "911 Alert",
            Self::Mayday => "Mayday",
            Self::RingTheBell => "Ring The Bell",
            Self::TroopsInContact => "Troops In Contact",
            Self::GeoFenceBreach => "Geo-fence Breach",
            Self::Cancel => "Cancel",
            Self::Custom(s) => s.as_str(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "911 Alert" => Self::Alert911,
            "Mayday" => Self::Mayday,
            "Ring The Bell" => Self::RingTheBell,
            "Troops In Contact" => Self::TroopsInContact,
            "Geo-fence Breach" => Self::GeoFenceBreach,
            "Cancel" => Self::Cancel,
            other => Self::Custom(other.to_string()),
        }
    }
}

impl EmergencyType {
    pub fn default_cot_type(&self) -> &'static str {
        match self {
            Self::Mayday => "b-a-o-pan",
            Self::Cancel => "b-a-o-can",
            _ => "b-a-o-tbl",
        }
    }
}
