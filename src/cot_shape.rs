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

/// Geographic vertex with optional height above ellipsoid (HAE).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CotVertex {
    pub lat: f64,
    pub lon: f64,
    pub hae: Option<f64>,
}

impl CotVertex {
    /// Create a 2D vertex validating latitude and longitude ranges.
    pub fn new(lat: f64, lon: f64) -> Result<Self, CotShapeError> {
        if !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
            return Err(CotShapeError::InvalidCoordinate("latitude in [-90, 90]"));
        }
        if !lon.is_finite() || !(-180.0..=180.0).contains(&lon) {
            return Err(CotShapeError::InvalidCoordinate("longitude in [-180, 180]"));
        }
        Ok(Self { lat, lon, hae: None })
    }
}
