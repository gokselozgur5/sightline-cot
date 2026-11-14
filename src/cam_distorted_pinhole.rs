//! Pinhole camera model with Brown-Conrady distortion parameters.

use crate::camera::BrownConrady;

/// Pinhole camera combining focal lengths, principal point and lens distortion.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DistortedPinhole {
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
    pub distortion: BrownConrady,
}
