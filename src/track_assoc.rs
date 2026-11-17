//! Multi-target track association and nearest-neighbor distance matrix solver.

/// Error conditions during track association and distance matrix solving.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssocError {
    NonFiniteInput,
    InvalidConfig,
    DimensionMismatch,
    IndexOutOfBounds,
}

impl std::fmt::Display for AssocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteInput => write!(f, "non-finite input encountered"),
            Self::InvalidConfig => write!(f, "invalid association configuration"),
            Self::DimensionMismatch => write!(f, "matrix dimension mismatch"),
            Self::IndexOutOfBounds => write!(f, "matrix index out of bounds"),
        }
    }
}

impl std::error::Error for AssocError {}

/// Configuration parameters for track-to-detection association.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AssocConfig {
    pub gate_threshold: f64,
    pub spatial_weight: f64,
    pub iou_weight: f64,
    pub cosine_weight: f64,
}

impl Default for AssocConfig {
    fn default() -> Self {
        Self {
            gate_threshold: 50.0,
            spatial_weight: 1.0,
            iou_weight: 10.0,
            cosine_weight: 5.0,
        }
    }
}

impl AssocConfig {
    pub fn validate(&self) -> Result<(), AssocError> {
        if !self.gate_threshold.is_finite()
            || !self.spatial_weight.is_finite()
            || !self.iou_weight.is_finite()
            || !self.cosine_weight.is_finite()
            || self.gate_threshold <= 0.0
            || self.spatial_weight < 0.0
            || self.iou_weight < 0.0
            || self.cosine_weight < 0.0
        {
            return Err(AssocError::InvalidConfig);
        }
        Ok(())
    }
}

/// 2D dense distance / cost matrix representation.
#[derive(Debug, Clone, PartialEq)]
pub struct DistanceMatrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl DistanceMatrix {
    pub fn new(rows: usize, cols: usize, fill: f64) -> Self {
        Self {
            rows,
            cols,
            data: vec![fill; rows * cols],
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl DistanceMatrix {
    pub fn get(&self, r: usize, c: usize) -> Option<f64> {
        if r < self.rows && c < self.cols {
            Some(self.data[r * self.cols + c])
        } else {
            None
        }
    }

    pub fn set(&mut self, r: usize, c: usize, val: f64) -> Result<(), AssocError> {
        if !val.is_finite() {
            return Err(AssocError::NonFiniteInput);
        }
        if r < self.rows && c < self.cols {
            self.data[r * self.cols + c] = val;
            Ok(())
        } else {
            Err(AssocError::IndexOutOfBounds)
        }
    }
}

impl DistanceMatrix {
    pub fn from_nested(nested: &[Vec<f64>]) -> Result<Self, AssocError> {
        let rows = nested.len();
        if rows == 0 {
            return Ok(Self::new(0, 0, 0.0));
        }
        let cols = nested[0].len();
        let mut data = Vec::with_capacity(rows * cols);
        for row in nested {
            if row.len() != cols {
                return Err(AssocError::DimensionMismatch);
            }
            for &val in row {
                if !val.is_finite() {
                    return Err(AssocError::NonFiniteInput);
                }
                data.push(val);
            }
        }
        Ok(Self { rows, cols, data })
    }
}

/// Compute Euclidean 2D distance between two points [x, y].
pub fn euclidean_dist_2d(p1: [f64; 2], p2: [f64; 2]) -> f64 {
    let dx = p1[0] - p2[0];
    let dy = p1[1] - p2[1];
    dx.hypot(dy)
}

/// Compute Euclidean 3D distance between two points [x, y, z].
pub fn euclidean_dist_3d(p1: [f64; 3], p2: [f64; 3]) -> f64 {
    let dx = p1[0] - p2[0];
    let dy = p1[1] - p2[1];
    let dz = p1[2] - p2[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Build pairwise 2D distance matrix between tracks and detections.
pub fn compute_spatial_matrix_2d(
    tracks: &[[f64; 2]],
    dets: &[[f64; 2]],
) -> Result<DistanceMatrix, AssocError> {
    for p in tracks.iter().chain(dets.iter()) {
        if !p[0].is_finite() || !p[1].is_finite() {
            return Err(AssocError::NonFiniteInput);
        }
    }
    let mut mat = DistanceMatrix::new(tracks.len(), dets.len(), 0.0);
    for (i, t) in tracks.iter().enumerate() {
        for (j, d) in dets.iter().enumerate() {
            let _ = mat.set(i, j, euclidean_dist_2d(*t, *d));
        }
    }
    Ok(mat)
}

/// Build pairwise 3D distance matrix between tracks and detections.
pub fn compute_spatial_matrix_3d(
    tracks: &[[f64; 3]],
    dets: &[[f64; 3]],
) -> Result<DistanceMatrix, AssocError> {
    for p in tracks.iter().chain(dets.iter()) {
        if !p[0].is_finite() || !p[1].is_finite() || !p[2].is_finite() {
            return Err(AssocError::NonFiniteInput);
        }
    }
    let mut mat = DistanceMatrix::new(tracks.len(), dets.len(), 0.0);
    for (i, t) in tracks.iter().enumerate() {
        for (j, d) in dets.iter().enumerate() {
            let _ = mat.set(i, j, euclidean_dist_3d(*t, *d));
        }
    }
    Ok(mat)
}

/// Single track-to-detection assignment match.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AssocMatch {
    pub track_idx: usize,
    pub det_idx: usize,
    pub cost: f64,
}

/// Association output containing matched pairs and unmatched indices.
#[derive(Debug, Clone, PartialEq)]
pub struct AssocResult {
    pub matches: Vec<AssocMatch>,
    pub unmatched_tracks: Vec<usize>,
    pub unmatched_dets: Vec<usize>,
}

/// Solve track association using greedy nearest-neighbor selection with gating.
pub fn solve_nearest_neighbor(mat: &DistanceMatrix, gate: f64) -> AssocResult {
    let mut candidates = Vec::new();
    for r in 0..mat.rows() {
        for c in 0..mat.cols() {
            if let Some(cost) = mat.get(r, c) {
                if cost <= gate {
                    candidates.push((cost, r, c));
                }
            }
        }
    }
    candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut matches = Vec::new();
    let mut matched_tracks = vec![false; mat.rows()];
    let mut matched_dets = vec![false; mat.cols()];

    for (cost, r, c) in candidates {
        if !matched_tracks[r] && !matched_dets[c] {
            matched_tracks[r] = true;
            matched_dets[c] = true;
            matches.push(AssocMatch {
                track_idx: r,
                det_idx: c,
                cost,
            });
        }
    }
    let unmatched_tracks = (0..mat.rows()).filter(|&i| !matched_tracks[i]).collect();
    let unmatched_dets = (0..mat.cols()).filter(|&j| !matched_dets[j]).collect();
    AssocResult {
        matches,
        unmatched_tracks,
        unmatched_dets,
    }
}
