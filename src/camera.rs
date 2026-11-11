//! Monocular camera models, SO(3) attitude, and ground ranging.

/// Errors occurring during camera back-projection and ground ranging.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraError {
    /// Ray does not intersect ground plane (horizontal or pointing upward).
    RayAboveHorizon,
    /// Numerical solver failed to invert fisheye distortion.
    DistortionSolverFailed,
    /// Invalid input parameters such as non-positive focal lengths.
    InvalidParameters,
}

impl std::fmt::Display for CameraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RayAboveHorizon => write!(f, "ray is above or parallel to horizon"),
            Self::DistortionSolverFailed => write!(f, "fisheye distortion solver failed"),
            Self::InvalidParameters => write!(f, "invalid camera or pose parameters"),
        }
    }
}

impl std::error::Error for CameraError {}

/// Camera intrinsic calibration model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Intrinsics {
    /// Standard rectilinear pinhole camera.
    Pinhole { fx: f64, fy: f64, cx: f64, cy: f64 },
    /// Equidistant fisheye model with radial polynomial distortion.
    EquidistantFisheye {
        fx: f64,
        fy: f64,
        cx: f64,
        cy: f64,
        k1: f64,
        k2: f64,
    },
}

impl Intrinsics {
    fn invert_equidistant(r_d: f64, k1: f64, k2: f64) -> Result<f64, CameraError> {
        let mut theta = r_d;
        for _ in 0..15 {
            let t2 = theta * theta;
            let f = theta * (1.0 + k1 * t2 + k2 * t2 * t2) - r_d;
            let df = 1.0 + 3.0 * k1 * t2 + 5.0 * k2 * t2 * t2;
            if df.abs() < 1e-12 {
                return Err(CameraError::DistortionSolverFailed);
            }
            let step = f / df;
            theta -= step;
            if step.abs() < 1e-12 {
                return Ok(theta);
            }
        }
        Err(CameraError::DistortionSolverFailed)
    }
}

impl Intrinsics {
    fn unproject_pinhole(
        u: f64,
        v: f64,
        fx: f64,
        fy: f64,
        cx: f64,
        cy: f64,
    ) -> Result<[f64; 3], CameraError> {
        if fx <= 0.0 || fy <= 0.0 {
            return Err(CameraError::InvalidParameters);
        }
        let x = (u - cx) / fx;
        let y = (v - cy) / fy;
        let norm = (x * x + y * y + 1.0).sqrt();
        Ok([x / norm, y / norm, 1.0 / norm])
    }
}

impl Intrinsics {
    fn unproject_fisheye(
        u: f64,
        v: f64,
        fx: f64,
        fy: f64,
        cx: f64,
        cy: f64,
        k1: f64,
        k2: f64,
    ) -> Result<[f64; 3], CameraError> {
        if fx <= 0.0 || fy <= 0.0 {
            return Err(CameraError::InvalidParameters);
        }
        let x_d = (u - cx) / fx;
        let y_d = (v - cy) / fy;
        let r_d = (x_d * x_d + y_d * y_d).sqrt();
        if r_d < 1e-12 {
            return Ok([0.0, 0.0, 1.0]);
        }
        let theta = Self::invert_equidistant(r_d, k1, k2)?;
        let scale = theta.sin() / r_d;
        Ok([x_d * scale, y_d * scale, theta.cos()])
    }
}
