use crate::camera::CameraError;

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
    if !fx.is_finite() || !fy.is_finite() || fx <= 0.0 || fy <= 0.0 || !cx.is_finite() || !cy.is_finite() {
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
    if !fx.is_finite() || !fy.is_finite() || fx <= 0.0 || fy <= 0.0 || !cx.is_finite() || !cy.is_finite() || !k1.is_finite() || !k2.is_finite() {
        return Err(ProjectionError::InvalidIntrinsics);
    }
    if !ray.iter().all(|x| x.is_finite()) {
        return Err(ProjectionError::InvalidRay);
    }
    let r_xy = (ray[0] * ray[0] + ray[1] * ray[1]).sqrt();
    let theta = r_xy.atan2(ray[2]);
    if theta <= 1e-12 {
        return Ok((cx, cy));
    }
    let theta2 = theta * theta;
    let r_d = theta * (1.0 + k1 * theta2 + k2 * theta2 * theta2);
    let scale = r_d / r_xy;
    Ok((cx + fx * ray[0] * scale, cy + fy * ray[1] * scale))
}
