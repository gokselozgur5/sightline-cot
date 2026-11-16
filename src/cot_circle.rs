//! CoT circle and range ring XML generator and validator.

/// Errors produced during CoT circle validation, parsing, or generation.
#[derive(Debug, Clone, PartialEq)]
pub enum CotCircleError {
    InvalidCoordinate(&'static str),
    InvalidRadius(&'static str),
    InvalidParameter(&'static str),
    XmlParseError(String),
}

impl std::fmt::Display for CotCircleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCoordinate(m) => write!(f, "invalid coordinate: {m}"),
            Self::InvalidRadius(m) => write!(f, "invalid radius: {m}"),
            Self::InvalidParameter(m) => write!(f, "invalid parameter: {m}"),
            Self::XmlParseError(m) => write!(f, "XML parse error: {m}"),
        }
    }
}

impl std::error::Error for CotCircleError {}

/// Cursor-on-Target circle or range ring shape representation.
#[derive(Debug, Clone, PartialEq)]
pub struct CotCircle {
    pub lat: f64,
    pub lon: f64,
    pub hae: Option<f64>,
    pub radius_m: f64,
    pub stroke_color: Option<i32>,
    pub fill_color: Option<i32>,
    pub stroke_weight: Option<f64>,
    pub callsign: Option<String>,
    pub remarks: Option<String>,
}
