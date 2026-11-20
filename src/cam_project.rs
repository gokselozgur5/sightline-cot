use crate::camera::{equidistant_monotonic_limit_sq, CameraError};

/// Error returned when forward camera projection fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionError {
    InvalidRay,
    BehindCamera,
    InvalidIntrinsics,
    DistortionFailed,
}

impl From<CameraError> for ProjectionError {
    fn from(err: CameraError) -> Self {
        match err {
            CameraError::InvalidParameters => ProjectionError::InvalidIntrinsics,
            CameraError::DistortionSolverFailed => ProjectionError::DistortionFailed,
            CameraError::RayAboveHorizon => ProjectionError::BehindCamera,
        }
    }
}

fn project_pinhole(
    ray: [f64; 3],
    fx: f64,
    fy: f64,
    cx: f64,
    cy: f64,
) -> Result<(f64, f64), ProjectionError> {
    if !fx.is_finite()
        || !fy.is_finite()
        || fx <= 0.0
        || fy <= 0.0
        || !cx.is_finite()
        || !cy.is_finite()
    {
        return Err(ProjectionError::InvalidIntrinsics);
    }
    if !ray.iter().all(|x| x.is_finite()) {
        return Err(ProjectionError::InvalidRay);
    }
    if ray[2] <= 0.0 {
        return Err(ProjectionError::BehindCamera);
    }
    let u = fx * (ray[0] / ray[2]) + cx;
    let v = fy * (ray[1] / ray[2]) + cy;
    Ok((u, v))
}

fn project_fisheye(
    ray: [f64; 3],
    fx: f64,
    fy: f64,
    cx: f64,
    cy: f64,
    k1: f64,
    k2: f64,
) -> Result<(f64, f64), ProjectionError> {
    if !fx.is_finite()
        || !fy.is_finite()
        || fx <= 0.0
        || fy <= 0.0
        || !cx.is_finite()
        || !cy.is_finite()
        || !k1.is_finite()
        || !k2.is_finite()
    {
        return Err(ProjectionError::InvalidIntrinsics);
    }
    if !ray.iter().all(|x| x.is_finite()) {
        return Err(ProjectionError::InvalidRay);
    }
    let r_xy = (ray[0] * ray[0] + ray[1] * ray[1]).sqrt();
    if r_xy <= 1e-12 {
        return if ray[2] < 0.0 {
            Err(ProjectionError::BehindCamera)
        } else {
            Ok((cx, cy))
        };
    }
    let theta = r_xy.atan2(ray[2]);
    let theta2 = theta * theta;
    if theta2 >= equidistant_monotonic_limit_sq(k1, k2) {
        return Err(ProjectionError::DistortionFailed);
    }
    let r_d = theta * (1.0 + k1 * theta2 + k2 * theta2 * theta2);
    let scale = r_d / r_xy;
    Ok((cx + fx * ray[0] * scale, cy + fy * ray[1] * scale))
}

use crate::camera::Intrinsics;

/// Forward project a unit ray in camera frame to pixel coordinates (u, v).
pub fn ray_to_pixel(intrinsics: &Intrinsics, ray: [f64; 3]) -> Result<(f64, f64), ProjectionError> {
    match *intrinsics {
        Intrinsics::Pinhole { fx, fy, cx, cy } => project_pinhole(ray, fx, fy, cx, cy),
        Intrinsics::EquidistantFisheye {
            fx,
            fy,
            cx,
            cy,
            k1,
            k2,
        } => project_fisheye(ray, fx, fy, cx, cy, k1, k2),
    }
}
