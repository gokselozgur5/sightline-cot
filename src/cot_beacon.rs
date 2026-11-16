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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EmergencySeverity {
    #[default]
    Critical,
    High,
    Medium,
    Low,
}

impl EmergencySeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmergencyBeacon {
    pub uid: String,
    pub callsign: String,
    pub emergency_type: EmergencyType,
    pub severity: EmergencySeverity,
    pub cancel: bool,
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub hae_m: f64,
    pub ce_m: f64,
    pub le_m: f64,
    pub time_ms: i64,
    pub stale_sec: i64,
    pub details: Option<String>,
}

fn validate_coords(lat: f64, lon: f64) -> Result<(), CotBeaconError> {
    if !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
        return Err(CotBeaconError::InvalidCoordinate("latitude in [-90, 90]"));
    }
    if !lon.is_finite() || !(-180.0..=180.0).contains(&lon) {
        return Err(CotBeaconError::InvalidCoordinate("longitude in [-180, 180]"));
    }
    Ok(())
}

impl EmergencyBeacon {
    pub fn new_911(uid: &str, callsign: &str, lat: f64, lon: f64) -> Result<Self, CotBeaconError> {
        validate_coords(lat, lon)?;
        Ok(Self {
            uid: uid.to_string(),
            callsign: callsign.to_string(),
            emergency_type: EmergencyType::Alert911,
            severity: EmergencySeverity::Critical,
            cancel: false,
            lat_deg: lat,
            lon_deg: lon,
            hae_m: 0.0,
            ce_m: 10.0,
            le_m: 10.0,
            time_ms: 0,
            stale_sec: 120,
            details: None,
        })
    }
}
