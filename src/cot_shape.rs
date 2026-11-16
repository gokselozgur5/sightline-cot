//! CoT shape polyline and polygon XML builder and parser.

#[derive(Debug, Clone, PartialEq)]
pub enum CotShapeError {
    InvalidCoordinate(&'static str),
    InsufficientVertices { expected: usize, found: usize },
    InvalidParameter(&'static str),
    XmlParseError(String),
}

impl std::fmt::Display for CotShapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCoordinate(m) => write!(f, "invalid coordinate: {m}"),
            Self::InsufficientVertices { expected, found } => {
                write!(f, "insufficient vertices: expected {expected}, got {found}")
            }
            Self::InvalidParameter(m) => write!(f, "invalid parameter: {m}"),
            Self::XmlParseError(m) => write!(f, "XML parse error: {m}"),
        }
    }
}

impl std::error::Error for CotShapeError {}
