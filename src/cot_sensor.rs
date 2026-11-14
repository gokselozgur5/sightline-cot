//! Sensor field-of-view CoT detail element.

use crate::cot::CotError;

/// Sensor FOV detail element in CoT 2.0 XML.
#[derive(Debug, Clone, PartialEq)]
pub struct CotSensor {
    pub azimuth_deg: f64,
    pub fov_deg: f64,
    pub range_m: f64,
    pub elevation_deg: f64,
}

impl CotSensor {
    /// Create a new CotSensor after validating FOV and parameters.
    pub fn new(az: f64, fov: f64, rng: f64, el: f64) -> Result<Self, CotError> {
        if !az.is_finite() || !fov.is_finite() || !rng.is_finite() || !el.is_finite() {
            return Err(CotError::InvalidNumber("sensor parameter"));
        }
        if !(0.0..=360.0).contains(&fov) {
            return Err(CotError::CoordinateOutOfBounds("fov in [0.0, 360.0]"));
        }
        if rng < 0.0 {
            return Err(CotError::CoordinateOutOfBounds("range >= 0.0"));
        }
        Ok(Self {
            azimuth_deg: az,
            fov_deg: fov,
            range_m: rng,
            elevation_deg: el,
        })
    }
}
