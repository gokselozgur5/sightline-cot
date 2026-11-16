//! CoT tactical task and route point XML builder and parser.

#[derive(Debug, Clone, PartialEq)]
pub enum CotTaskError {
    InvalidCoordinate(&'static str),
    InvalidParameter(&'static str),
    EmptyRoute,
    InsufficientPoints { expected: usize, found: usize },
    XmlParseError(String),
    MissingField(&'static str),
}

impl std::fmt::Display for CotTaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCoordinate(m) => write!(f, "invalid coordinate: {m}"),
            Self::InvalidParameter(m) => write!(f, "invalid parameter: {m}"),
            Self::EmptyRoute => write!(f, "tactical task has no route points"),
            Self::InsufficientPoints { expected, found } => {
                write!(f, "insufficient points: expected {expected}, got {found}")
            }
            Self::XmlParseError(m) => write!(f, "XML parse error: {m}"),
            Self::MissingField(m) => write!(f, "missing field: {m}"),
        }
    }
}

impl std::error::Error for CotTaskError {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TaskAction {
    #[default]
    FlyOver,
    FlyBy,
    Ingress,
    Egress,
    Orbit,
    Drop,
    Landing,
    Target,
    Checkpoint,
    Rendezvous,
    Custom(String),
}

impl TaskAction {
    pub fn as_str(&self) -> &str {
        match self {
            Self::FlyOver => "flyover",
            Self::FlyBy => "flyby",
            Self::Ingress => "ingress",
            Self::Egress => "egress",
            Self::Orbit => "orbit",
            Self::Drop => "drop",
            Self::Landing => "landing",
            Self::Target => "target",
            Self::Checkpoint => "checkpoint",
            Self::Rendezvous => "rendezvous",
            Self::Custom(s) => s.as_str(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "flyover" => Self::FlyOver,
            "flyby" => Self::FlyBy,
            "ingress" => Self::Ingress,
            "egress" => Self::Egress,
            "orbit" => Self::Orbit,
            "drop" => Self::Drop,
            "landing" => Self::Landing,
            "target" => Self::Target,
            "checkpoint" => Self::Checkpoint,
            "rendezvous" => Self::Rendezvous,
            other => Self::Custom(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskPriority {
    #[default]
    Routine,
    Priority,
    Immediate,
    Flash,
}

impl TaskPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Routine => "routine",
            Self::Priority => "priority",
            Self::Immediate => "immediate",
            Self::Flash => "flash",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "priority" => Self::Priority,
            "immediate" => Self::Immediate,
            "flash" => Self::Flash,
            _ => Self::Routine,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskStatus {
    #[default]
    Planned,
    InProgress,
    Completed,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "in_progress" => Self::InProgress,
            "completed" => Self::Completed,
            "cancelled" => Self::Cancelled,
            _ => Self::Planned,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaskPoint {
    pub order: u32,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub hae: Option<f64>,
    pub action: TaskAction,
    pub speed_mps: Option<f64>,
    pub eta_ms: Option<i64>,
    pub remarks: Option<String>,
}

fn validate_lat_lon(lat: f64, lon: f64) -> Result<(), CotTaskError> {
    if !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
        return Err(CotTaskError::InvalidCoordinate("latitude in [-90, 90]"));
    }
    if !lon.is_finite() || !(-180.0..=180.0).contains(&lon) {
        return Err(CotTaskError::InvalidCoordinate("longitude in [-180, 180]"));
    }
    Ok(())
}

fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    let needle = format!("{attr}=\"");
    let start = tag.find(&needle)? + needle.len();
    let end = tag[start..].find('"')?;
    Some(tag[start..start + end].to_string())
}

impl TaskPoint {
    pub fn new(order: u32, name: impl Into<String>, lat: f64, lon: f64) -> Result<Self, CotTaskError> {
        validate_lat_lon(lat, lon)?;
        Ok(Self {
            order,
            name: name.into(),
            lat,
            lon,
            hae: None,
            action: TaskAction::default(),
            speed_mps: None,
            eta_ms: None,
            remarks: None,
        })
    }

    pub fn with_hae(mut self, hae: f64) -> Result<Self, CotTaskError> {
        if !hae.is_finite() {
            return Err(CotTaskError::InvalidParameter("hae must be finite"));
        }
        self.hae = Some(hae);
        Ok(self)
    }

    pub fn with_action(mut self, action: TaskAction) -> Self {
        self.action = action;
        self
    }

    pub fn with_speed(mut self, speed_mps: f64) -> Result<Self, CotTaskError> {
        if !speed_mps.is_finite() || speed_mps < 0.0 {
            return Err(CotTaskError::InvalidParameter("speed_mps must be non-negative"));
        }
        self.speed_mps = Some(speed_mps);
        Ok(self)
    }

    pub fn with_eta(mut self, eta_ms: i64) -> Self {
        self.eta_ms = Some(eta_ms);
        self
    }

    pub fn with_remarks(mut self, remarks: impl Into<String>) -> Self {
        self.remarks = Some(remarks.into());
        self
    }
