//! CoT Sensor FOV Frustum and camera sensor polygon footprint generator.

/// Errors occurring during sensor frustum calculations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrustumError {
    NonFiniteParameter(&'static str),
    InvalidFov(&'static str),
    InvalidAltitude(&'static str),
    InvalidRange(&'static str),
    NoGroundIntersection,
    DegenerateFrustum,
    XmlParseError(String),
}

impl core::fmt::Display for FrustumError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFiniteParameter(s) => write!(f, "parameter is non-finite: {s}"),
            Self::InvalidFov(s) => write!(f, "invalid FOV angle: {s}"),
            Self::InvalidAltitude(s) => write!(f, "invalid altitude: {s}"),
            Self::InvalidRange(s) => write!(f, "invalid range: {s}"),
            Self::NoGroundIntersection => write!(f, "sensor frustum does not intersect ground"),
            Self::DegenerateFrustum => write!(f, "degenerate sensor frustum geometry"),
            Self::XmlParseError(s) => write!(f, "XML parse error: {s}"),
        }
    }
}

impl std::error::Error for FrustumError {}


/// Position and orientation of the sensor host platform.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlatformPose {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub alt_hae_m: f64,
    pub heading_deg: f64,
    pub pitch_deg: f64,
    pub roll_deg: f64,
}

impl PlatformPose {
    /// Create and validate a new PlatformPose.
    pub fn new(lat: f64, lon: f64, alt: f64, hdg: f64, pitch: f64, roll: f64) -> Result<Self, FrustumError> {
        if !lat.is_finite() || !lon.is_finite() || !alt.is_finite()
            || !hdg.is_finite() || !pitch.is_finite() || !roll.is_finite() {
            return Err(FrustumError::NonFiniteParameter("platform pose"));
        }
        if !(-90.0..=90.0).contains(&lat) {
            return Err(FrustumError::InvalidAltitude("latitude out of range"));
        }
        let hdg_norm = (hdg % 360.0 + 360.0) % 360.0;
        Ok(Self { lat_deg: lat, lon_deg: lon, alt_hae_m: alt, heading_deg: hdg_norm, pitch_deg: pitch, roll_deg: roll })
    }
}
