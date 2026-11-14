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

/// Compute slant range and grazing angle from 3D sensor and target positions.
pub fn compute_slant_range_and_grazing(
    sensor_pos_m: [f64; 3],
    target_pos_m: [f64; 3],
) -> Result<SlantRangeGeometry, CameraError> {
    if !sensor_pos_m.iter().chain(target_pos_m.iter()).all(|x| x.is_finite()) {
        return Err(CameraError::InvalidParameters);
    }
    let de = target_pos_m[0] - sensor_pos_m[0];
    let dn = target_pos_m[1] - sensor_pos_m[1];
    let du = target_pos_m[2] - sensor_pos_m[2];
    let ground_range_m = (de * de + dn * dn).sqrt();
    let slant_range_m = (ground_range_m * ground_range_m + du * du).sqrt();
    if slant_range_m < 1e-6 {
        return Err(CameraError::InvalidParameters);
    }
    let elevation_angle_rad = du.atan2(ground_range_m);
    let grazing_angle_rad = (-du).atan2(ground_range_m);
    Ok(SlantRangeGeometry {
        slant_range_m,
        ground_range_m,
        grazing_angle_rad,
        elevation_angle_rad,
    })
}
