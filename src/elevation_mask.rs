//! Tactical line-of-sight terrain elevation masking angle validator.

/// Errors occurring during elevation mask validation and calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElevationMaskError {
    /// Non-finite coordinate or angle value.
    NonFiniteValue,
    /// Azimuth angle is outside [0.0, 360.0) degrees.
    InvalidAzimuth,
    /// Elevation angle is outside [-90.0, 90.0] degrees.
    InvalidElevation,
    /// Distance or range is non-positive or non-finite.
    InvalidDistance,
    /// Obstacle profile has insufficient points.
    EmptyProfile,
    /// Sector angular span is invalid.
    InvalidSectorSpan,
    /// Profile distance sequence is non-monotonic.
    NonMonotonicProfile,
}

impl core::fmt::Display for ElevationMaskError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFiniteValue => write!(f, "coordinate or angle value is non-finite"),
            Self::InvalidAzimuth => write!(f, "azimuth must be in range [0.0, 360.0) degrees"),
            Self::InvalidElevation => write!(f, "elevation must be in range [-90.0, 90.0] degrees"),
            Self::InvalidDistance => write!(f, "distance must be positive and finite"),
            Self::EmptyProfile => write!(f, "obstacle profile cannot be empty"),
            Self::InvalidSectorSpan => write!(f, "azimuth sector span is invalid"),
            Self::NonMonotonicProfile => write!(f, "obstacle profile distances must strictly increase"),
        }
    }
}

impl std::error::Error for ElevationMaskError {}

/// Line-of-sight terrain masking classification status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskingStatus {
    /// Line-of-sight is unobstructed with positive clearance margin.
    Clear,
    /// Line-of-sight is within clearance margin threshold of obstruction.
    Grazing,
    /// Line-of-sight is blocked by terrain or obstruction.
    Masked,
}

impl MaskingStatus {
    /// Return true if the line-of-sight is not fully masked.
    pub fn is_visible(&self) -> bool {
        matches!(self, Self::Clear | Self::Grazing)
    }

    /// Return true if line-of-sight has clear positive clearance.
    pub fn is_clear(&self) -> bool {
        matches!(self, Self::Clear)
    }
}

/// Result of evaluating line-of-sight against terrain elevation mask.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaskingEvaluation {
    /// Target azimuth in degrees [0.0, 360.0).
    pub target_azimuth_deg: f64,
    /// Target elevation angle in degrees [-90.0, 90.0].
    pub target_elevation_deg: f64,
    /// Minimum required masking elevation angle in degrees.
    pub mask_elevation_deg: f64,
    /// Angular clearance margin in degrees (target - mask).
    pub clearance_margin_deg: f64,
    /// Classified masking status.
    pub status: MaskingStatus,
}

impl MaskingEvaluation {
    /// Return true if the evaluation indicates visible line-of-sight.
    pub fn is_visible(&self) -> bool {
        self.status.is_visible()
    }
}

/// Azimuth sector defining terrain masking elevation angle boundary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaskSector {
    /// Start azimuth of sector in degrees [0.0, 360.0).
    pub start_azimuth_deg: f64,
    /// End azimuth of sector in degrees [0.0, 360.0).
    pub end_azimuth_deg: f64,
    /// Minimum line-of-sight elevation angle in degrees [-90.0, 90.0].
    pub mask_elevation_deg: f64,
}

impl MaskSector {
    /// Create and validate an azimuth masking sector.
    pub fn new(start_az: f64, end_az: f64, mask_el: f64) -> Result<Self, ElevationMaskError> {
        if !start_az.is_finite() || !end_az.is_finite() || !mask_el.is_finite() {
            return Err(ElevationMaskError::NonFiniteValue);
        }
        if !(0.0..360.0).contains(&start_az) || !(0.0..360.0).contains(&end_az) {
            return Err(ElevationMaskError::InvalidAzimuth);
        }
        if !(-90.0..=90.0).contains(&mask_el) {
            return Err(ElevationMaskError::InvalidElevation);
        }
        Ok(Self { start_azimuth_deg: start_az, end_azimuth_deg: end_az, mask_elevation_deg: mask_el })
    }

    /// Check if an azimuth angle falls within this sector.
    pub fn contains_azimuth(&self, az_deg: f64) -> bool {
        if !az_deg.is_finite() {
            return false;
        }
        let az = (az_deg % 360.0 + 360.0) % 360.0;
        if self.start_azimuth_deg <= self.end_azimuth_deg {
            az >= self.start_azimuth_deg && az <= self.end_azimuth_deg
        } else {
            az >= self.start_azimuth_deg || az <= self.end_azimuth_deg
        }
    }
}
