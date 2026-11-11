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

#[allow(clippy::needless_range_loop)]
fn mat4_mul(a: [[f64; 4]; 4], b: [[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut out = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                out[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    out
}

impl Track {
    fn transition_matrix(dt_s: f64) -> [[f64; 4]; 4] {
        let mut f = [[0.0; 4]; 4];
        for i in 0..4 {
            f[i][i] = 1.0;
        }
        f[0][2] = dt_s;
        f[1][3] = dt_s;
        f
    }
}

impl Track {
    /// Predict track state forward by dt_s seconds.
    pub fn predict(&mut self, dt_s: f64, q_acc: f64) {
        self.x[0] += self.x[2] * dt_s;
        self.x[1] += self.x[3] * dt_s;
        let f = Self::transition_matrix(dt_s);
        let mut ft = f;
        ft[0][2] = 0.0;
        ft[2][0] = dt_s;
        ft[1][3] = 0.0;
        ft[3][1] = dt_s;
        self.p = mat4_mul(mat4_mul(f, self.p), ft);
        let (dt2, dt3, dt4) = (dt_s * dt_s, dt_s.powi(3) / 2.0, dt_s.powi(4) / 4.0);
        self.p[0][0] += dt4 * q_acc;
        self.p[0][2] += dt3 * q_acc;
        self.p[2][0] += dt3 * q_acc;
        self.p[2][2] += dt2 * q_acc;
        self.p[1][1] += dt4 * q_acc;
        self.p[1][3] += dt3 * q_acc;
        self.p[3][1] += dt3 * q_acc;
        self.p[3][3] += dt2 * q_acc;
        self.age += 1;
        self.time_since_update += 1;
    }
}

impl Track {
    /// Compute 2x2 innovation covariance matrix S = H P H^T + R and its determinant.
    fn compute_s_matrix(&self, r_var: f64) -> ([[f64; 2]; 2], f64) {
        let s00 = self.p[0][0] + r_var;
        let s01 = self.p[0][1];
        let s10 = self.p[1][0];
        let s11 = self.p[1][1] + r_var;
        let det = s00 * s11 - s01 * s10;
        ([[s00, s01], [s10, s11]], det)
    }

    /// Invert 2x2 innovation matrix S with guard against singularity.
    fn invert_s(s: [[f64; 2]; 2], det: f64) -> Option<[[f64; 2]; 2]> {
        if !det.is_finite() || det.abs() < 1e-12 {
            return None;
        }
        let inv_det = 1.0 / det;
        Some([
            [s[1][1] * inv_det, -s[0][1] * inv_det],
            [-s[1][0] * inv_det, s[0][0] * inv_det],
        ])
    }
}
