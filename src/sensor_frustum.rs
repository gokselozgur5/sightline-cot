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


/// Compute 3x3 rotation matrix from yaw (azimuth clockwise from North), pitch, and roll in ENU.
pub fn euler_to_rot_matrix(yaw_deg: f64, pitch_deg: f64, roll_deg: f64) -> [[f64; 3]; 3] {
    let (s_y, c_y) = yaw_deg.to_radians().sin_cos();
    let (s_p, c_p) = pitch_deg.to_radians().sin_cos();
    let (s_r, c_r) = roll_deg.to_radians().sin_cos();
    [
        [c_y * c_r + s_y * s_p * s_r, -s_y * c_p, c_y * s_r - s_y * s_p * c_r],
        [s_y * c_r - c_y * s_p * s_r, c_y * c_p, s_y * s_r + c_y * s_p * c_r],
        [-c_p * s_r, -s_p, c_p * c_r],
    ]
}

/// Multiply a 3x3 matrix by a 3D vector.
pub fn rotate_vec(m: &[[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}


/// Compute 4 normalized corner ray vectors [Top-Left, Top-Right, Bottom-Right, Bottom-Left] in sensor body frame.
pub fn compute_frustum_corner_rays(h_fov_deg: f64, v_fov_deg: f64) -> [[f64; 3]; 4] {
    let tan_h = (h_fov_deg * 0.5).to_radians().tan();
    let tan_v = (v_fov_deg * 0.5).to_radians().tan();
    let corners = [
        [-tan_h, 1.0, tan_v],
        [tan_h, 1.0, tan_v],
        [tan_h, 1.0, -tan_v],
        [-tan_h, 1.0, -tan_v],
    ];
    let mut out = [[0.0; 3]; 4];
    for (i, c) in corners.iter().enumerate() {
        let norm = (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt();
        out[i] = [c[0] / norm, c[1] / norm, c[2] / norm];
    }
    out
}


/// Intersect a ray with the flat ground plane (Z = 0 in local ENU).
pub fn intersect_ray_ground(
    ray_enu: [f64; 3],
    sensor_h: f64,
    max_range_m: f64,
) -> Result<([f64; 3], f64), FrustumError> {
    if sensor_h <= 0.0 {
        return Err(FrustumError::InvalidAltitude("sensor altitude must be above ground"));
    }
    if ray_enu[2] >= -1e-6 {
        return Err(FrustumError::NoGroundIntersection);
    }
    let t = -sensor_h / ray_enu[2];
    if t > max_range_m {
        return Err(FrustumError::InvalidRange("intersection exceeds max range"));
    }
    let p = [t * ray_enu[0], t * ray_enu[1], 0.0];
    Ok((p, t))
}


/// Ground footprint polygon generated by sensor frustum intersection.
#[derive(Debug, Clone, PartialEq)]
pub struct FrustumFootprint {
    pub vertices_enu: Vec<[f64; 3]>,
    pub vertices_geo: Vec<[f64; 3]>,
    pub boresight_geo: [f64; 3],
    pub area_sq_m: f64,
    pub perimeter_m: f64,
    pub slant_range_m: f64,
}

impl FrustumFootprint {
    /// Calculate centroid in geodetic coordinates [lat, lon, alt].
    pub fn centroid_geo(&self) -> [f64; 3] {
        let n = self.vertices_geo.len() as f64;
        let lat = self.vertices_geo.iter().map(|p| p[0]).sum::<f64>() / n;
        let lon = self.vertices_geo.iter().map(|p| p[1]).sum::<f64>() / n;
        let alt = self.vertices_geo.iter().map(|p| p[2]).sum::<f64>() / n;
        [lat, lon, alt]
    }
}
