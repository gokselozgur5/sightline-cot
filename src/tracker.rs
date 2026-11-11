//! 3D Constant-Velocity Kalman Tracker.

/// Error types occurring during multi-target tracking operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackerError {
    /// Non-finite numerical state or detection coordinates encountered.
    NonFiniteInput,
    /// Invalid time delta (negative or non-finite).
    InvalidDeltaTime,
}

impl std::fmt::Display for TrackerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteInput => write!(f, "non-finite input encountered"),
            Self::InvalidDeltaTime => write!(f, "invalid delta time"),
        }
    }
}

impl std::error::Error for TrackerError {}

/// 2D bounding box in pixel coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

impl BoundingBox {
    /// Compute Intersection-over-Union (IoU) between two bounding boxes.
    pub fn iou(&self, other: &BoundingBox) -> f64 {
        let (ix1, iy1) = (self.x1.max(other.x1), self.y1.max(other.y1));
        let (ix2, iy2) = (self.x2.min(other.x2), self.y2.min(other.y2));
        let inter = (ix2 - ix1).max(0.0) * (iy2 - iy1).max(0.0);
        let a1 = (self.x2 - self.x1).max(0.0) * (self.y2 - self.y1).max(0.0);
        let a2 = (other.x2 - other.x1).max(0.0) * (other.y2 - other.y1).max(0.0);
        let union = a1 + a2 - inter;
        if union > 1e-12 {
            inter / union
        } else {
            0.0
        }
    }
}

/// Cosine distance between two 8-dimensional L2-normalized embeddings.
pub fn cosine_distance(a: &[f32; 8], b: &[f32; 8]) -> f64 {
    let mut dot = 0.0f64;
    for i in 0..8 {
        dot += (a[i] as f64) * (b[i] as f64);
    }
    (1.0 - dot).clamp(0.0, 2.0)
}
