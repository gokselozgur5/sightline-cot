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
