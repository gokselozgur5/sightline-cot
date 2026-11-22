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
    /// Compute combined association cost from spatial distance, bounding box IoU, and embedding cosine distance.
    pub fn compute_cost(&self, spatial_dist: f64, iou: f64, cosine_dist: f64) -> f64 {
        self.spatial_weight * spatial_dist
            + self.iou_weight * (1.0 - iou).max(0.0)
            + self.cosine_weight * cosine_dist
    }
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
            mat.set(i, j, euclidean_dist_2d(*t, *d))?;
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
            mat.set(i, j, euclidean_dist_3d(*t, *d))?;
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

/// Solve optimal bipartite track association using Hungarian algorithm with gating.
pub fn solve_optimal_hungarian(mat: &DistanceMatrix, gate: f64) -> AssocResult {
    let n = mat.rows();
    let m = mat.cols();
    if n == 0 || m == 0 {
        return AssocResult {
            matches: Vec::new(),
            unmatched_tracks: (0..n).collect(),
            unmatched_dets: (0..m).collect(),
        };
    }
    let k = n.max(m);
    let dummy = 1e9;
    let mut a = vec![vec![0.0; k + 1]; k + 1];
    for i in 0..k {
        for j in 0..k {
            let val = if i < n && j < m {
                let cost = mat.get(i, j).unwrap_or(dummy);
                if cost <= gate {
                    cost
                } else {
                    dummy
                }
            } else {
                dummy
            };
            a[i + 1][j + 1] = val;
        }
    }

    let mut u = vec![0.0; k + 1];
    let mut v = vec![0.0; k + 1];
    let mut p = vec![0usize; k + 1];
    let mut way = vec![0usize; k + 1];

    for i in 1..=k {
        p[0] = i;
        let mut j0 = 0usize;
        let mut minv = vec![f64::INFINITY; k + 1];
        let mut used = vec![false; k + 1];
        loop {
            used[j0] = true;
            let i0 = p[j0];
            let mut delta = f64::INFINITY;
            let mut j1 = 0usize;

            for j in 1..=k {
                if !used[j] {
                    let cur = a[i0][j] - u[i0] - v[j];
                    if cur < minv[j] {
                        minv[j] = cur;
                        way[j] = j0;
                    }
                    if minv[j] < delta {
                        delta = minv[j];
                        j1 = j;
                    }
                }
            }
            for j in 0..=k {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }
            j0 = j1;
            if p[j0] == 0 {
                break;
            }
        }

        loop {
            let j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
            if j0 == 0 {
                break;
            }
        }
    }

    let mut matches = Vec::new();
    let mut matched_tracks = vec![false; n];
    let mut matched_dets = vec![false; m];
    for (j, &i) in p.iter().enumerate().take(m + 1).skip(1) {
        if i >= 1 && i <= n {
            let r = i - 1;
            let c = j - 1;
            if let Some(cost) = mat.get(r, c) {
                if cost <= gate {
                    matched_tracks[r] = true;
                    matched_dets[c] = true;
                    matches.push(AssocMatch {
                        track_idx: r,
                        det_idx: c,
                        cost,
                    });
                }
            }
        }
    }
    matches.sort_by_key(|m| m.track_idx);

    let unmatched_tracks = (0..n).filter(|&i| !matched_tracks[i]).collect();
    let unmatched_dets = (0..m).filter(|&j| !matched_dets[j]).collect();
    AssocResult {
        matches,
        unmatched_tracks,
        unmatched_dets,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assoc_error_and_config() {
        let err = AssocError::InvalidConfig;
        assert_eq!(err.to_string(), "invalid association configuration");
        let cfg = AssocConfig::default();
        assert!(cfg.validate().is_ok());
        let bad = AssocConfig {
            gate_threshold: -1.0,
            ..cfg
        };
        assert_eq!(bad.validate(), Err(AssocError::InvalidConfig));
    }

    #[test]
    fn test_distance_matrix_ops() {
        let mut mat = DistanceMatrix::new(2, 2, 1.0);
        assert_eq!(mat.rows(), 2);
        assert_eq!(mat.cols(), 2);
        assert_eq!(mat.get(0, 0), Some(1.0));
        assert!(mat.set(0, 1, 5.0).is_ok());
        assert_eq!(mat.get(0, 1), Some(5.0));
        assert_eq!(mat.get(2, 2), None);
        assert_eq!(mat.set(3, 0, 1.0), Err(AssocError::IndexOutOfBounds));
        assert_eq!(mat.set(0, 0, f64::NAN), Err(AssocError::NonFiniteInput));
    }

    #[test]
    fn test_from_nested_and_spatial() {
        let nested = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let mat = DistanceMatrix::from_nested(&nested).unwrap();
        assert_eq!(mat.get(1, 0), Some(3.0));
        let bad = vec![vec![1.0, 2.0], vec![3.0]];
        assert_eq!(
            DistanceMatrix::from_nested(&bad),
            Err(AssocError::DimensionMismatch)
        );

        let t2 = [[0.0, 0.0], [10.0, 10.0]];
        let d2 = [[0.0, 3.0], [10.0, 14.0]];
        let smat = compute_spatial_matrix_2d(&t2, &d2).unwrap();
        assert!((smat.get(0, 0).unwrap() - 3.0).abs() < 1e-9);
        assert!((smat.get(1, 1).unwrap() - 4.0).abs() < 1e-9);
    }

    #[test]
    fn test_spatial_matrix_3d_and_helpers() {
        assert_eq!(euclidean_dist_2d([0.0, 0.0], [3.0, 4.0]), 5.0);
        assert_eq!(euclidean_dist_3d([0.0, 0.0, 0.0], [1.0, 2.0, 2.0]), 3.0);
        let t3 = [[0.0, 0.0, 0.0]];
        let d3 = [[1.0, 2.0, 2.0]];
        let m3 = compute_spatial_matrix_3d(&t3, &d3).unwrap();
        assert_eq!(m3.get(0, 0), Some(3.0));
    }

    #[test]
    fn test_solve_nearest_neighbor_greedy() {
        let nested = vec![vec![10.0, 20.0], vec![20.0, 5.0]];
        let mat = DistanceMatrix::from_nested(&nested).unwrap();
        let res = solve_nearest_neighbor(&mat, 15.0);
        assert_eq!(res.matches.len(), 2);
        assert_eq!(res.unmatched_tracks.len(), 0);
        assert_eq!(res.unmatched_dets.len(), 0);
        let res_gated = solve_nearest_neighbor(&mat, 8.0);
        assert_eq!(res_gated.matches.len(), 1);
        assert_eq!(res_gated.matches[0].track_idx, 1);
        assert_eq!(res_gated.matches[0].det_idx, 1);
    }

    #[test]
    fn test_solve_optimal_hungarian_optimality() {
        let nested = vec![vec![10.0, 11.0], vec![12.0, 100.0]];
        let mat = DistanceMatrix::from_nested(&nested).unwrap();
        let res = solve_optimal_hungarian(&mat, 50.0);
        assert_eq!(res.matches.len(), 2);
        assert_eq!(
            res.matches[0],
            AssocMatch {
                track_idx: 0,
                det_idx: 1,
                cost: 11.0
            }
        );
        assert_eq!(
            res.matches[1],
            AssocMatch {
                track_idx: 1,
                det_idx: 0,
                cost: 12.0
            }
        );
    }

    #[test]
    fn test_hungarian_rectangular_and_empty() {
        let empty = DistanceMatrix::new(0, 0, 0.0);
        let res_empty = solve_optimal_hungarian(&empty, 10.0);
        assert!(res_empty.matches.is_empty());
        let rect = vec![vec![5.0, 10.0], vec![10.0, 5.0], vec![20.0, 20.0]];
        let mat_rect = DistanceMatrix::from_nested(&rect).unwrap();
        let res = solve_optimal_hungarian(&mat_rect, 15.0);
        assert_eq!(res.matches.len(), 2);
        assert_eq!(res.unmatched_tracks, vec![2]);
        assert!(res.unmatched_dets.is_empty());
    }
}
