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

/// Single 3D detection input to the multi-target tracker.
#[derive(Debug, Clone, PartialEq)]
pub struct Detection {
    pub east_m: f64,
    pub north_m: f64,
    pub hae_m: f64,
    pub ce90_m: f64,
    pub le90_m: f64,
    pub bbox: BoundingBox,
    pub embedding: [f32; 8],
}

/// Track state enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackStatus {
    Tentative,
    Confirmed,
    Coasting,
    Deleted,
}

/// Active 3D Kalman track representation.
#[derive(Debug, Clone, PartialEq)]
pub struct Track {
    pub id: u64,
    pub x: [f64; 4],
    pub p: [[f64; 4]; 4],
    pub hae_m: f64,
    pub bbox: BoundingBox,
    pub embedding: [f32; 8],
    pub hits: usize,
    pub age: usize,
    pub time_since_update: usize,
    pub status: TrackStatus,
}

impl Track {
    /// Create a new tentative track initialized from a detection.
    pub fn new(id: u64, det: &Detection) -> Self {
        let var_xy = (det.ce90_m / 2.146).powi(2).max(1.0);
        let mut p = [[0.0; 4]; 4];
        p[0][0] = var_xy;
        p[1][1] = var_xy;
        p[2][2] = 100.0;
        p[3][3] = 100.0;

        Self {
            id,
            x: [det.east_m, det.north_m, 0.0, 0.0],
            p,
            hae_m: det.hae_m,
            bbox: det.bbox,
            embedding: det.embedding,
            hits: 1,
            age: 1,
            time_since_update: 0,
            status: TrackStatus::Tentative,
        }
    }
}
