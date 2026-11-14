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

use crate::camera::{CameraPose, Intrinsics};
use crate::geo_bbox::GeoBbox;

/// Axis-aligned bounding box in 3D ENU frame [min_east, min_north, min_up] to [max_east, max_north, max_up].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnuBbox {
    pub min_e: f64,
    pub min_n: f64,
    pub min_u: f64,
    pub max_e: f64,
    pub max_n: f64,
    pub max_u: f64,
}

impl EnuBbox {
    /// Create an EnuBbox enclosing a slice of 3D points.
    pub fn from_points(points: &[[f64; 3]]) -> Option<Self> {
        let first = points.first()?;
        let (mut min_e, mut max_e) = (first[0], first[0]);
        let (mut min_n, mut max_n) = (first[1], first[1]);
        let (mut min_u, mut max_u) = (first[2], first[2]);
        for p in &points[1..] {
            min_e = min_e.min(p[0]);
            max_e = max_e.max(p[0]);
            min_n = min_n.min(p[1]);
            max_n = max_n.max(p[1]);
            min_u = min_u.min(p[2]);
            max_u = max_u.max(p[2]);
        }
        Some(Self {
            min_e,
            min_n,
            min_u,
            max_e,
            max_n,
            max_u,
        })
    }
}
