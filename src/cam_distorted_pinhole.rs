//! Pinhole camera model with Brown-Conrady distortion parameters.

use crate::camera::{BrownConrady, CameraError};

/// Pinhole camera combining focal lengths, principal point and lens distortion.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DistortedPinhole {
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
    pub distortion: BrownConrady,
}

impl DistortedPinhole {
    /// Create a new distorted pinhole model with parameter validation.
    pub fn new(
        fx: f64,
        fy: f64,
        cx: f64,
        cy: f64,
        dist: BrownConrady,
    ) -> Result<Self, CameraError> {
        let k = [dist.k1, dist.k2, dist.k3, dist.p1, dist.p2];
        let f_ok = fx.is_finite() && fx > 0.0 && fy.is_finite() && fy > 0.0;
        let p_ok = cx.is_finite() && cy.is_finite() && k.iter().all(|c| c.is_finite());
        if !f_ok || !p_ok {
            return Err(CameraError::InvalidParameters);
        }
        Ok(Self {
            fx,
            fy,
            cx,
            cy,
            distortion: dist,
        })
    }
}
