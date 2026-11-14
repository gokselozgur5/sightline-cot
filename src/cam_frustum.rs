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

impl EnuBbox {
    /// Check whether a 3D ENU point lies within or on the bounding box.
    pub fn contains(&self, p: [f64; 3]) -> bool {
        p[0] >= self.min_e
            && p[0] <= self.max_e
            && p[1] >= self.min_n
            && p[1] <= self.max_n
            && p[2] >= self.min_u
            && p[2] <= self.max_u
    }

    /// Center of the 3D bounding box.
    pub fn center(&self) -> [f64; 3] {
        [
            (self.min_e + self.max_e) * 0.5,
            (self.min_n + self.max_n) * 0.5,
            (self.min_u + self.max_u) * 0.5,
        ]
    }
}

/// Camera frustum bounding volume in local ENU and geodetic frames.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraFrustum {
    pub corners_cam: [[f64; 3]; 8],
    pub corners_enu: [[f64; 3]; 8],
    pub enu_bbox: EnuBbox,
    pub geo_bbox: GeoBbox,
}

use crate::camera::enu_to_geodetic;

fn check_inputs(w: f64, h: f64, n: f64, f: f64, lat: f64, lon: f64, hae: f64) -> bool {
    w.is_finite() && w > 0.0 && h.is_finite() && h > 0.0
        && n.is_finite() && n > 0.0 && f.is_finite() && f > n
        && lat.is_finite() && (-90.0..=90.0).contains(&lat)
        && lon.is_finite() && hae.is_finite()
}
