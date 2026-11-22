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

impl DistortedPinhole {
    /// Back-project a pixel coordinate (u, v) into a unit camera-frame ray.
    pub fn pixel_to_ray(&self, u: f64, v: f64) -> Result<[f64; 3], CameraError> {
        if !u.is_finite() || !v.is_finite() {
            return Err(CameraError::InvalidParameters);
        }
        let xd = (u - self.cx) / self.fx;
        let yd = (v - self.cy) / self.fy;
        let (x, y) = self.distortion.undistort(xd, yd, 20)?;
        let norm = x.hypot(y).hypot(1.0);
        if !norm.is_finite() {
            return Err(CameraError::InvalidParameters);
        }
        Ok([x / norm, y / norm, 1.0 / norm])
    }

    /// Forward project a unit camera-frame ray to pixel coordinates (u, v).
    pub fn ray_to_pixel(&self, ray: [f64; 3]) -> Result<(f64, f64), CameraError> {
        if !ray.iter().all(|x| x.is_finite()) || ray[2] <= 0.0 {
            return Err(CameraError::InvalidParameters);
        }
        let x = ray[0] / ray[2];
        let y = ray[1] / ray[2];
        let (xd, yd) = self.distortion.distort(x, y);
        let u = self.fx * xd + self.cx;
        let v = self.fy * yd + self.cy;
        Ok((u, v))
    }
}
