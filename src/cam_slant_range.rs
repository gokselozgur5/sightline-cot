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

impl SlantRangeGeometry {
    /// Return the grazing angle in degrees.
    pub fn grazing_angle_deg(&self) -> f64 {
        self.grazing_angle_rad.to_degrees()
    }

    /// Return the elevation angle in degrees.
    pub fn elevation_angle_deg(&self) -> f64 {
        self.elevation_angle_rad.to_degrees()
    }
}
