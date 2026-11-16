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
