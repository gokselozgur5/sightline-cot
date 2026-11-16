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

impl CotCircle {
    /// Create a new circle shape with center coordinate and radius in meters.
    pub fn new(lat: f64, lon: f64, radius_m: f64) -> Result<Self, CotCircleError> {
        if !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
            return Err(CotCircleError::InvalidCoordinate("latitude in [-90, 90]"));
        }
        if !lon.is_finite() || !(-180.0..=180.0).contains(&lon) {
            return Err(CotCircleError::InvalidCoordinate("longitude in [-180, 180]"));
        }
        if !radius_m.is_finite() || radius_m <= 0.0 {
            return Err(CotCircleError::InvalidRadius("radius must be positive"));
        }
        Ok(Self {
            lat,
            lon,
            hae: None,
            radius_m,
            stroke_color: None,
            fill_color: None,
            stroke_weight: None,
            callsign: None,
            remarks: None,
        })
    }
}
