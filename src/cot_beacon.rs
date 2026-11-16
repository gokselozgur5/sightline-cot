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
