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


/// Gimbal pointing angles relative to platform frame or North-referenced.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorGimbal {
    pub azimuth_deg: f64,
    pub elevation_deg: f64,
    pub roll_deg: f64,
}

impl SensorGimbal {
    /// Create and validate a SensorGimbal pointing angles.
    pub fn new(az: f64, el: f64, roll: f64) -> Result<Self, FrustumError> {
        if !az.is_finite() || !el.is_finite() || !roll.is_finite() {
            return Err(FrustumError::NonFiniteParameter("sensor gimbal"));
        }
        if !(-90.0..=90.0).contains(&el) {
            return Err(FrustumError::InvalidAltitude("elevation [-90, 90]"));
        }
        let az_norm = (az % 360.0 + 360.0) % 360.0;
        Ok(Self { azimuth_deg: az_norm, elevation_deg: el, roll_deg: roll })
    }
}


/// Configuration parameters defining the sensor field-of-view frustum.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorFrustumConfig {
    pub h_fov_deg: f64,
    pub v_fov_deg: f64,
    pub max_range_m: f64,
    pub min_range_m: f64,
}

impl SensorFrustumConfig {
    /// Create and validate a new SensorFrustumConfig.
    pub fn new(h_fov: f64, v_fov: f64, max_rng: f64, min_rng: f64) -> Result<Self, FrustumError> {
        if !h_fov.is_finite() || !v_fov.is_finite() || !max_rng.is_finite() || !min_rng.is_finite() {
            return Err(FrustumError::NonFiniteParameter("frustum config"));
        }
        if h_fov <= 0.0 || h_fov >= 180.0 {
            return Err(FrustumError::InvalidFov("horizontal FOV in (0, 180)"));
        }
        if v_fov <= 0.0 || v_fov >= 180.0 {
            return Err(FrustumError::InvalidFov("vertical FOV in (0, 180)"));
        }
        if min_rng < 0.0 || max_rng <= min_rng {
            return Err(FrustumError::InvalidRange("max_range must be > min_range >= 0"));
        }
        Ok(Self { h_fov_deg: h_fov, v_fov_deg: v_fov, max_range_m: max_rng, min_range_m: min_rng })
    }
}
