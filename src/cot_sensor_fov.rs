//! CoT sensor field-of-view (FOV) frustum and cone calculator.

/// Errors occurring during sensor FOV calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorFovError {
    NonFiniteParameter,
    InvalidFovAngle,
    InvalidRange,
    InvalidElevation,
    RayDoesNotIntersectGround,
}

impl core::fmt::Display for SensorFovError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFiniteParameter => write!(f, "parameter is non-finite"),
            Self::InvalidFovAngle => write!(f, "FOV angle must be in (0.0, 180.0) degrees"),
            Self::InvalidRange => write!(f, "range must be strictly positive"),
            Self::InvalidElevation => write!(f, "elevation must be in [-90.0, 90.0] degrees"),
            Self::RayDoesNotIntersectGround => write!(f, "sensor ray does not intersect ground"),
        }
    }
}

impl std::error::Error for SensorFovError {}

/// Spherical cone parameters representing CoT sensor field of view.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorCone {
    pub azimuth_deg: f64,
    pub elevation_deg: f64,
    pub fov_deg: f64,
    pub range_m: f64,
}

impl SensorCone {
    /// Create and validate a new SensorCone.
    pub fn new(az_deg: f64, el_deg: f64, fov_deg: f64, range_m: f64) -> Result<Self, SensorFovError> {
        if !az_deg.is_finite() || !el_deg.is_finite() || !fov_deg.is_finite() || !range_m.is_finite() {
            return Err(SensorFovError::NonFiniteParameter);
        }
        if fov_deg <= 0.0 || fov_deg >= 180.0 {
            return Err(SensorFovError::InvalidFovAngle);
        }
        if range_m <= 0.0 {
            return Err(SensorFovError::InvalidRange);
        }
        if !(-90.0..=90.0).contains(&el_deg) {
            return Err(SensorFovError::InvalidElevation);
        }
        let norm_az = (az_deg % 360.0 + 360.0) % 360.0;
        Ok(Self { azimuth_deg: norm_az, elevation_deg: el_deg, fov_deg, range_m })
    }
}

impl SensorCone {
    /// Compute unit boresight vector in local ENU frame [East, North, Up].
    pub fn boresight_vector(&self) -> [f64; 3] {
        let az_rad = self.azimuth_deg.to_radians();
        let el_rad = self.elevation_deg.to_radians();
        let cos_el = el_rad.cos();
        let sin_el = el_rad.sin();
        let east = az_rad.sin() * cos_el;
        let north = az_rad.cos() * cos_el;
        let up = sin_el;
        [east, north, up]
    }
}

impl SensorCone {
    /// Determine if a relative target point [east, north, up] in meters lies inside the cone.
    pub fn contains_relative_point(&self, rel_enu: [f64; 3]) -> bool {
        let [e, n, u] = rel_enu;
        let dist = (e * e + n * n + u * u).sqrt();
        if dist < 1e-9 {
            return true;
        }
        if dist > self.range_m {
            return false;
        }
        let b = self.boresight_vector();
        let cos_angle = (e * b[0] + n * b[1] + u * b[2]) / dist;
        let half_fov_rad = (self.fov_deg * 0.5).to_radians();
        cos_angle >= half_fov_rad.cos() - 1e-12
    }
}

/// 3D ground footprint produced by intersecting sensor FOV cone with ground plane.
#[derive(Debug, Clone, PartialEq)]
pub struct SensorFootprint {
    pub boresight_ground: [f64; 3],
    pub perimeter_points: Vec<[f64; 3]>,
    pub slant_range_boresight_m: f64,
}
