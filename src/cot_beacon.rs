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

impl EmergencyBeacon {
    pub fn new_mayday(uid: &str, callsign: &str, lat: f64, lon: f64) -> Result<Self, CotBeaconError> {
        validate_coords(lat, lon)?;
        Ok(Self {
            uid: uid.to_string(),
            callsign: callsign.to_string(),
            emergency_type: EmergencyType::Mayday,
            severity: EmergencySeverity::Critical,
            cancel: false,
            lat_deg: lat,
            lon_deg: lon,
            hae_m: 0.0,
            ce_m: 10.0,
            le_m: 10.0,
            time_ms: 0,
            stale_sec: 180,
            details: None,
        })
    }
}

impl EmergencyBeacon {
    pub fn new_cancel(uid: &str, callsign: &str, lat: f64, lon: f64) -> Result<Self, CotBeaconError> {
        validate_coords(lat, lon)?;
        Ok(Self {
            uid: uid.to_string(),
            callsign: callsign.to_string(),
            emergency_type: EmergencyType::Cancel,
            severity: EmergencySeverity::Low,
            cancel: true,
            lat_deg: lat,
            lon_deg: lon,
            hae_m: 0.0,
            ce_m: 10.0,
            le_m: 10.0,
            time_ms: 0,
            stale_sec: 60,
            details: None,
        })
    }
}

impl EmergencyBeacon {
    pub fn with_hae(mut self, hae_m: f64) -> Self {
        self.hae_m = hae_m;
        self
    }
    pub fn with_details(mut self, details: &str) -> Self {
        self.details = Some(details.to_string());
        self
    }
    pub fn with_stale_duration(mut self, sec: i64) -> Self {
        self.stale_sec = sec;
        self
    }
    pub fn with_time(mut self, time_ms: i64) -> Self {
        self.time_ms = time_ms;
        self
    }
}

fn escape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

fn format_iso8601(epoch_ms: i64) -> String {
    let total_secs = epoch_ms / 1000;
    let days = total_secs / 86400;
    let rem_secs = total_secs % 86400;
    let hours = rem_secs / 3600;
    let mins = (rem_secs % 3600) / 60;
    let secs = rem_secs % 60;
    let (year, month, day) = days_to_ymd(days);
    format!("{year:04}-{month:02}-{day:02}T{hours:02}:{mins:02}:{secs:02}Z")
}

fn days_to_ymd(days: i64) -> (i64, i64, i64) {
    let mut y = 1970 + days / 365;
    let mut d = days % 365;
    if d < 0 {
        d += 365;
        y -= 1;
    }
    (y, 1, (d % 28) + 1)
}

impl EmergencyBeacon {
    pub fn to_cot_xml(&self) -> String {
        let time_str = format_iso8601(self.time_ms);
        let stale_str = format_iso8601(self.time_ms + self.stale_sec * 1000);
        let cot_type = self.emergency_type.default_cot_type();
        let desc = self.details.as_deref().unwrap_or(self.emergency_type.as_str());
        format!(
            "<event version=\"2.0\" uid=\"{uid}\" type=\"{cot_type}\" how=\"m-g\" time=\"{time_str}\" start=\"{time_str}\" stale=\"{stale_str}\"><point lat=\"{lat:.7}\" lon=\"{lon:.7}\" hae=\"{hae:.2}\" ce=\"{ce:.2}\" le=\"{le:.2}\"/><detail><contact callsign=\"{callsign}\"/><emergency cancel=\"{cancel}\" type=\"{etype}\">{desc}</emergency></detail></event>",
            uid = escape_xml(&self.uid),
            cot_type = cot_type,
            time_str = time_str,
            stale_str = stale_str,
            lat = self.lat_deg,
            lon = self.lon_deg,
            hae = self.hae_m,
            ce = self.ce_m,
            le = self.le_m,
            callsign = escape_xml(&self.callsign),
            cancel = self.cancel,
            etype = escape_xml(self.emergency_type.as_str()),
            desc = escape_xml(desc),
        )
    }
}

fn extract_attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let key = format!("{name}=\"");
    let start = tag.find(&key)? + key.len();
    let end = tag[start..].find('\"')? + start;
    Some(&tag[start..end])
}

fn extract_tag_content<'a>(xml: &'a str, tag: &str) -> Option<&'a str> {
    let open = format!("<{tag}");
    let start = xml.find(&open)?;
    let content_start = xml[start..].find('>')? + start + 1;
    let close = format!("</{tag}>");
    let end = xml[content_start..].find(&close)? + content_start;
    Some(&xml[content_start..end])
}

fn parse_point_coords(xml: &str) -> Result<(f64, f64), CotBeaconError> {
    let lat_s = extract_attr(xml, "lat").ok_or(CotBeaconError::MissingField("lat"))?;
    let lon_s = extract_attr(xml, "lon").ok_or(CotBeaconError::MissingField("lon"))?;
    let lat: f64 = lat_s.parse().map_err(|_| CotBeaconError::InvalidCoordinate("lat"))?;
    let lon: f64 = lon_s.parse().map_err(|_| CotBeaconError::InvalidCoordinate("lon"))?;
    validate_coords(lat, lon)?;
    Ok((lat, lon))
}

impl EmergencyBeacon {
    pub fn from_cot_xml(xml: &str) -> Result<Self, CotBeaconError> {
        let uid = extract_attr(xml, "uid").ok_or(CotBeaconError::MissingField("uid"))?;
        let (lat, lon) = parse_point_coords(xml)?;
        let callsign = extract_attr(xml, "callsign").unwrap_or("UNKNOWN");
        let cancel = extract_attr(xml, "cancel").map(|c| c == "true").unwrap_or(false);
        let etype_s = extract_attr(xml, "type").unwrap_or("911 Alert");
        let desc = extract_tag_content(xml, "emergency").map(|s| s.to_string());
        Ok(Self {
            uid: uid.to_string(),
            callsign: callsign.to_string(),
            emergency_type: EmergencyType::from_str(etype_s),
            severity: if cancel { EmergencySeverity::Low } else { EmergencySeverity::Critical },
            cancel,
            lat_deg: lat,
            lon_deg: lon,
            hae_m: extract_attr(xml, "hae").and_then(|s| s.parse().ok()).unwrap_or(0.0),
            ce_m: extract_attr(xml, "ce").and_then(|s| s.parse().ok()).unwrap_or(10.0),
            le_m: extract_attr(xml, "le").and_then(|s| s.parse().ok()).unwrap_or(10.0),
            time_ms: 0,
            stale_sec: 120,
            details: desc,
        })
    }
}
