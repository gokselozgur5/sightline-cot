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
