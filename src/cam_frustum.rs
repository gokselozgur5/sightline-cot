//! Camera frustum bounding volume generation in local ENU and geodetic frames.

use crate::camera::CameraError;

/// Error returned when camera frustum bounding volume computation fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrustumError {
    InvalidParameters,
    Camera(CameraError),
}

impl std::fmt::Display for FrustumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidParameters => write!(f, "invalid frustum parameters"),
            Self::Camera(err) => write!(f, "camera error: {err}"),
        }
    }
}

impl std::error::Error for FrustumError {}

impl From<CameraError> for FrustumError {
    fn from(err: CameraError) -> Self {
        Self::Camera(err)
    }
}
