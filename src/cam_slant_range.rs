//! Slant range and grazing angle calculation for sensor-target geometry.

use crate::camera::CameraError;

/// Slant geometry outcome containing 3D range, ground range, and angles.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlantRangeGeometry {
    pub slant_range_m: f64,
    pub ground_range_m: f64,
    pub grazing_angle_rad: f64,
    pub elevation_angle_rad: f64,
}
