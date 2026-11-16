//! Line-of-sight elevation and slant-range calculator.

/// Errors occurring during line-of-sight calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeoLosError {
    NonFiniteCoordinate,
    LatitudeOutOfRange,
    CoincidentPoints,
}

impl core::fmt::Display for GeoLosError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFiniteCoordinate => write!(f, "coordinate is non-finite"),
            Self::LatitudeOutOfRange => write!(f, "latitude must be between -90 and 90 degrees"),
            Self::CoincidentPoints => write!(f, "observer and target positions are coincident"),
        }
    }
}

impl std::error::Error for GeoLosError {}

/// 3D geographic point specified by latitude, longitude, and height above ellipsoid.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoPoint3D {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub hae_m: f64,
}

impl GeoPoint3D {
    /// Create and validate a geographic 3D position.
    pub fn new(lat_deg: f64, lon_deg: f64, hae_m: f64) -> Result<Self, GeoLosError> {
        if !lat_deg.is_finite() || !lon_deg.is_finite() || !hae_m.is_finite() {
            return Err(GeoLosError::NonFiniteCoordinate);
        }
        if !(-90.0..=90.0).contains(&lat_deg) {
            return Err(GeoLosError::LatitudeOutOfRange);
        }
        Ok(Self { lat_deg, lon_deg, hae_m })
    }
}

/// Result of a 3D line-of-sight calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LosGeometry {
    pub slant_range_m: f64,
    pub ground_range_m: f64,
    pub elevation_angle_rad: f64,
    pub azimuth_rad: f64,
    pub horizon_distance_m: f64,
    pub is_direct_los: bool,
}

impl LosGeometry {
    /// Return the elevation angle in degrees.
    pub fn elevation_angle_deg(&self) -> f64 {
        self.elevation_angle_rad.to_degrees()
    }

    /// Return the azimuth (true bearing) in degrees in range [0, 360).
    pub fn azimuth_deg(&self) -> f64 {
        let deg = self.azimuth_rad.to_degrees();
        (deg % 360.0 + 360.0) % 360.0
    }
}

pub const WGS84_MEAN_RADIUS_M: f64 = 6_371_000.0;

/// Compute geometric horizon line-of-sight limit distance between two heights.
pub fn horizon_distance_m(h1_m: f64, h2_m: f64) -> Result<f64, GeoLosError> {
    if !h1_m.is_finite() || !h2_m.is_finite() {
        return Err(GeoLosError::NonFiniteCoordinate);
    }
    let d1 = if h1_m > 0.0 { (2.0 * WGS84_MEAN_RADIUS_M * h1_m + h1_m * h1_m).sqrt() } else { 0.0 };
    let d2 = if h2_m > 0.0 { (2.0 * WGS84_MEAN_RADIUS_M * h2_m + h2_m * h2_m).sqrt() } else { 0.0 };
    Ok(d1 + d2)
}
