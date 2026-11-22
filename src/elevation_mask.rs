//! Tactical line-of-sight terrain elevation masking angle validator.

use crate::los::earth_curvature_drop;

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
            Self::NonMonotonicProfile => {
                write!(f, "obstacle profile distances must strictly increase")
            }
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
        if !(0.0..=360.0).contains(&start_az) || !(0.0..=360.0).contains(&end_az) {
            return Err(ElevationMaskError::InvalidAzimuth);
        }
        if !(-90.0..=90.0).contains(&mask_el) {
            return Err(ElevationMaskError::InvalidElevation);
        }
        if (end_az - start_az).abs() >= 360.0 {
            return Err(ElevationMaskError::InvalidSectorSpan);
        }
        let start = (start_az % 360.0 + 360.0) % 360.0;
        let end = (end_az % 360.0 + 360.0) % 360.0;
        Ok(Self {
            start_azimuth_deg: start,
            end_azimuth_deg: end,
            mask_elevation_deg: mask_el,
        })
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

/// Discrete terrain or obstacle point sampled along radial line-of-sight.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObstaclePoint {
    /// Radial ground distance from observer in meters.
    pub distance_m: f64,
    /// Terrain surface elevation in meters above observer datum.
    pub elevation_m: f64,
    /// Structural or canopy height above ground level in meters.
    pub height_agl_m: f64,
}

impl ObstaclePoint {
    /// Create and validate a radial obstacle sample point.
    pub fn new(
        distance_m: f64,
        elevation_m: f64,
        height_agl_m: f64,
    ) -> Result<Self, ElevationMaskError> {
        if !distance_m.is_finite() || !elevation_m.is_finite() || !height_agl_m.is_finite() {
            return Err(ElevationMaskError::NonFiniteValue);
        }
        if distance_m <= 0.0 {
            return Err(ElevationMaskError::InvalidDistance);
        }
        if height_agl_m < 0.0 {
            return Err(ElevationMaskError::NonFiniteValue);
        }
        Ok(Self {
            distance_m,
            elevation_m,
            height_agl_m,
        })
    }
}

/// Profile of terrain obstacle samples along an azimuth radial.
#[derive(Debug, Clone, PartialEq)]
pub struct ObstacleProfile {
    /// Ordered sequence of obstacle sample points.
    pub points: Vec<ObstaclePoint>,
}

impl ObstacleProfile {
    /// Create and validate an obstacle profile along a radial.
    pub fn new(points: Vec<ObstaclePoint>) -> Result<Self, ElevationMaskError> {
        if points.is_empty() {
            return Err(ElevationMaskError::EmptyProfile);
        }
        for i in 1..points.len() {
            if points[i].distance_m <= points[i - 1].distance_m {
                return Err(ElevationMaskError::NonMonotonicProfile);
            }
        }
        Ok(Self { points })
    }

    /// Compute maximum masking elevation angle in degrees along profile.
    pub fn compute_mask_angle_deg(
        &self,
        observer_height_m: f64,
        include_earth_curvature: bool,
    ) -> Result<f64, ElevationMaskError> {
        self.compute_mask_angle_up_to_deg(observer_height_m, f64::INFINITY, include_earth_curvature)
    }

    /// Compute maximum masking elevation angle in degrees along profile up to max_dist_m.
    pub fn compute_mask_angle_up_to_deg(
        &self,
        observer_height_m: f64,
        max_dist_m: f64,
        include_earth_curvature: bool,
    ) -> Result<f64, ElevationMaskError> {
        if !observer_height_m.is_finite() || !max_dist_m.is_finite() && !max_dist_m.is_infinite() {
            return Err(ElevationMaskError::NonFiniteValue);
        }
        let mut max_angle_rad = -std::f64::consts::FRAC_PI_2;
        for pt in &self.points {
            if pt.distance_m > max_dist_m {
                break;
            }
            let drop_m = if include_earth_curvature {
                earth_curvature_drop(pt.distance_m)
            } else {
                0.0
            };
            let top_m = pt.elevation_m + pt.height_agl_m - drop_m;
            let angle_rad = (top_m - observer_height_m).atan2(pt.distance_m);
            if angle_rad > max_angle_rad {
                max_angle_rad = angle_rad;
            }
        }
        Ok(max_angle_rad.to_degrees())
    }
}

/// Tactical line-of-sight terrain elevation masking validator.
#[derive(Debug, Clone, PartialEq)]
pub struct ElevationMaskValidator {
    /// Observer altitude / height in meters above datum.
    pub observer_height_m: f64,
    /// Default masking elevation angle in degrees for uncovered sectors.
    pub default_mask_deg: f64,
    /// Angular tolerance margin in degrees for grazing classification.
    pub margin_threshold_deg: f64,
    /// Configured azimuth masking sectors.
    pub sectors: Vec<MaskSector>,
}

impl ElevationMaskValidator {
    /// Create a new elevation mask validator.
    pub fn new(observer_height_m: f64, default_mask_deg: f64) -> Result<Self, ElevationMaskError> {
        if !observer_height_m.is_finite() || !default_mask_deg.is_finite() {
            return Err(ElevationMaskError::NonFiniteValue);
        }
        if !(-90.0..=90.0).contains(&default_mask_deg) {
            return Err(ElevationMaskError::InvalidElevation);
        }
        Ok(Self {
            observer_height_m,
            default_mask_deg,
            margin_threshold_deg: 0.5,
            sectors: Vec::new(),
        })
    }

    /// Configure grazing margin threshold in degrees.
    pub fn with_margin_threshold(mut self, margin_deg: f64) -> Result<Self, ElevationMaskError> {
        if !margin_deg.is_finite() || margin_deg < 0.0 {
            return Err(ElevationMaskError::NonFiniteValue);
        }
        self.margin_threshold_deg = margin_deg;
        Ok(self)
    }

    /// Add an azimuth masking sector to validator.
    pub fn add_sector(&mut self, sector: MaskSector) {
        self.sectors.push(sector);
    }

    /// Return the minimum required masking elevation angle at specified azimuth.
    pub fn get_mask_angle_at_azimuth(&self, azimuth_deg: f64) -> Result<f64, ElevationMaskError> {
        if !azimuth_deg.is_finite() {
            return Err(ElevationMaskError::NonFiniteValue);
        }
        if !(0.0..360.0).contains(&azimuth_deg) {
            return Err(ElevationMaskError::InvalidAzimuth);
        }
        for sector in &self.sectors {
            if sector.contains_azimuth(azimuth_deg) {
                return Ok(sector.mask_elevation_deg);
            }
        }
        Ok(self.default_mask_deg)
    }

    /// Validate line-of-sight elevation clearance against sector mask.
    pub fn validate_target_angles(
        &self,
        az_deg: f64,
        el_deg: f64,
    ) -> Result<MaskingEvaluation, ElevationMaskError> {
        if !el_deg.is_finite() {
            return Err(ElevationMaskError::NonFiniteValue);
        }
        if !(-90.0..=90.0).contains(&el_deg) {
            return Err(ElevationMaskError::InvalidElevation);
        }
        let mask_el = self.get_mask_angle_at_azimuth(az_deg)?;
        let margin = el_deg - mask_el;
        let status = if margin > self.margin_threshold_deg {
            MaskingStatus::Clear
        } else if margin >= -self.margin_threshold_deg {
            MaskingStatus::Grazing
        } else {
            MaskingStatus::Masked
        };
        Ok(MaskingEvaluation {
            target_azimuth_deg: az_deg,
            target_elevation_deg: el_deg,
            mask_elevation_deg: mask_el,
            clearance_margin_deg: margin,
            status,
        })
    }

    /// Compute target elevation angle from observer to target with curvature option.
    pub fn compute_target_elevation_deg(
        &self,
        dist_m: f64,
        height_m: f64,
        curvature: bool,
    ) -> Result<f64, ElevationMaskError> {
        if !dist_m.is_finite() || !height_m.is_finite() {
            return Err(ElevationMaskError::NonFiniteValue);
        }
        if dist_m <= 0.0 {
            return Err(ElevationMaskError::InvalidDistance);
        }
        let drop_m = if curvature {
            earth_curvature_drop(dist_m)
        } else {
            0.0
        };
        Ok(((height_m - drop_m - self.observer_height_m).atan2(dist_m)).to_degrees())
    }

    /// Validate line-of-sight elevation against radial obstacle profile.
    pub fn validate_target_profile(
        &self,
        az_deg: f64,
        dist_m: f64,
        height_m: f64,
        profile: &ObstacleProfile,
        curvature: bool,
    ) -> Result<MaskingEvaluation, ElevationMaskError> {
        let target_el = self.compute_target_elevation_deg(dist_m, height_m, curvature)?;
        let prof_mask =
            profile.compute_mask_angle_up_to_deg(self.observer_height_m, dist_m, curvature)?;
        let sect_mask = self.get_mask_angle_at_azimuth(az_deg)?;
        let eff_mask = prof_mask.max(sect_mask);
        let margin = target_el - eff_mask;
        let status = if margin > self.margin_threshold_deg {
            MaskingStatus::Clear
        } else if margin >= -self.margin_threshold_deg {
            MaskingStatus::Grazing
        } else {
            MaskingStatus::Masked
        };
        Ok(MaskingEvaluation {
            target_azimuth_deg: az_deg,
            target_elevation_deg: target_el,
            mask_elevation_deg: eff_mask,
            clearance_margin_deg: margin,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_and_input_validations() {
        assert_eq!(
            MaskSector::new(f64::NAN, 10.0, 5.0),
            Err(ElevationMaskError::NonFiniteValue)
        );
        assert_eq!(
            MaskSector::new(360.5, 10.0, 5.0),
            Err(ElevationMaskError::InvalidAzimuth)
        );
        assert_eq!(
            MaskSector::new(0.0, 10.0, 95.0),
            Err(ElevationMaskError::InvalidElevation)
        );
        assert_eq!(
            ObstaclePoint::new(0.0, 10.0, 0.0),
            Err(ElevationMaskError::InvalidDistance)
        );
        assert_eq!(
            ObstacleProfile::new(vec![]),
            Err(ElevationMaskError::EmptyProfile)
        );
        let p1 = ObstaclePoint::new(100.0, 10.0, 0.0).unwrap();
        let p2 = ObstaclePoint::new(50.0, 10.0, 0.0).unwrap();
        assert_eq!(
            ObstacleProfile::new(vec![p1, p2]),
            Err(ElevationMaskError::NonMonotonicProfile)
        );
    }

    #[test]
    fn test_sectors_and_target_validation() {
        let mut val = ElevationMaskValidator::new(10.0, 2.0).unwrap();
        val.add_sector(MaskSector::new(350.0, 20.0, 8.0).unwrap());
        val.add_sector(MaskSector::new(45.0, 90.0, 15.0).unwrap());
        assert!((val.get_mask_angle_at_azimuth(0.0).unwrap() - 8.0).abs() < 1e-6);
        assert!((val.get_mask_angle_at_azimuth(180.0).unwrap() - 2.0).abs() < 1e-6);

        let eval_clear = val.validate_target_angles(60.0, 20.0).unwrap();
        assert_eq!(eval_clear.status, MaskingStatus::Clear);
        assert!(eval_clear.is_visible());

        let eval_grazing = val.validate_target_angles(60.0, 15.2).unwrap();
        assert_eq!(eval_grazing.status, MaskingStatus::Grazing);

        let eval_masked = val.validate_target_angles(60.0, 10.0).unwrap();
        assert_eq!(eval_masked.status, MaskingStatus::Masked);
        assert!(!eval_masked.is_visible());
    }

    #[test]
    fn test_obstacle_profile_and_mask_computation() {
        let p1 = ObstaclePoint::new(1000.0, 50.0, 10.0).unwrap();
        let p2 = ObstaclePoint::new(2000.0, 120.0, 0.0).unwrap();
        let prof = ObstacleProfile::new(vec![p1, p2]).unwrap();
        let angle_flat = prof.compute_mask_angle_deg(10.0, false).unwrap();
        assert!(angle_flat > 0.0);

        let val = ElevationMaskValidator::new(10.0, 0.0).unwrap();
        let res = val
            .validate_target_profile(90.0, 3000.0, 250.0, &prof, false)
            .unwrap();
        assert_eq!(res.status, MaskingStatus::Clear);

        let res_blocked = val
            .validate_target_profile(90.0, 3000.0, 50.0, &prof, false)
            .unwrap();
        assert_eq!(res_blocked.status, MaskingStatus::Masked);
    }
}
