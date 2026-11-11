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
