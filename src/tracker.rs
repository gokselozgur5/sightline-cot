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
        if !self.x1.is_finite()
            || !self.y1.is_finite()
            || !self.x2.is_finite()
            || !self.y2.is_finite()
            || !other.x1.is_finite()
            || !other.y1.is_finite()
            || !other.x2.is_finite()
            || !other.y2.is_finite()
        {
            return 0.0;
        }
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

pub fn normalize_embedding(emb: &[f32; 8]) -> [f32; 8] {
    let sum_sq: f32 = emb.iter().map(|x| x * x).sum();
    let norm = sum_sq.sqrt();
    if !norm.is_finite() || norm < 1e-9 {
        [0.0; 8]
    } else {
        let mut out = [0.0; 8];
        for i in 0..8 {
            out[i] = emb[i] / norm;
        }
        out
    }
}

/// Cosine distance between two 8-dimensional embeddings.
pub fn cosine_distance(a: &[f32; 8], b: &[f32; 8]) -> f64 {
    let na = normalize_embedding(a);
    let nb = normalize_embedding(b);
    let mut dot = 0.0f64;
    for i in 0..8 {
        dot += (na[i] as f64) * (nb[i] as f64);
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
        let var_xy = (det.ce90_m / 2.146).powi(2).max(1e-6);
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
            embedding: normalize_embedding(&det.embedding),
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
    #[allow(clippy::needless_range_loop)]
    fn symmetrize_p(&mut self) {
        for i in 0..4 {
            for j in (i + 1)..4 {
                let avg = 0.5 * (self.p[i][j] + self.p[j][i]);
                self.p[i][j] = avg;
                self.p[j][i] = avg;
            }
        }
    }

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
        self.symmetrize_p();
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
        if !det.is_finite() || det <= 1e-12 || s[0][0] <= 0.0 || s[1][1] <= 0.0 {
            return None;
        }
        let inv_det = 1.0 / det;
        Some([
            [s[1][1] * inv_det, -s[0][1] * inv_det],
            [-s[1][0] * inv_det, s[0][0] * inv_det],
        ])
    }
}

impl Track {
    /// Compute squared Mahalanobis distance d2 = y^T S^-1 y for gated association.
    pub fn mahalanobis_d2(&self, det: &Detection) -> Option<f64> {
        let r_var = (det.ce90_m / 2.146).powi(2).max(1e-6);
        let (s, det_s) = self.compute_s_matrix(r_var);
        let s_inv = Self::invert_s(s, det_s)?;

        let y = [det.east_m - self.x[0], det.north_m - self.x[1]];
        let d2 = y[0] * (s_inv[0][0] * y[0] + s_inv[0][1] * y[1])
            + y[1] * (s_inv[1][0] * y[0] + s_inv[1][1] * y[1]);

        if d2.is_finite() && d2 >= 0.0 {
            Some(d2)
        } else {
            None
        }
    }
}

impl Track {
    #[allow(clippy::needless_range_loop)]
    fn compute_kalman_gain(&self, s_inv: [[f64; 2]; 2]) -> [[f64; 2]; 4] {
        let mut k = [[0.0; 2]; 4];
        for i in 0..4 {
            k[i][0] = self.p[i][0] * s_inv[0][0] + self.p[i][1] * s_inv[1][0];
            k[i][1] = self.p[i][0] * s_inv[0][1] + self.p[i][1] * s_inv[1][1];
        }
        k
    }
}

impl Track {
    #[allow(clippy::needless_range_loop)]
    fn apply_joseph_update(&mut self, k: [[f64; 2]; 4], r_var: f64) {
        let mut ikh = [[0.0; 4]; 4];
        for i in 0..4 {
            ikh[i][i] = 1.0;
            ikh[i][0] -= k[i][0];
            ikh[i][1] -= k[i][1];
        }
        let mut p_new = mat4_mul(mat4_mul(ikh, self.p), {
            let mut ikht = [[0.0; 4]; 4];
            for i in 0..4 {
                for j in 0..4 {
                    ikht[i][j] = ikh[j][i];
                }
            }
            ikht
        });
        for i in 0..4 {
            for j in 0..4 {
                p_new[i][j] += (k[i][0] * k[j][0] + k[i][1] * k[j][1]) * r_var;
            }
        }
        self.p = p_new;
        self.symmetrize_p();
    }
}

impl Track {
    /// Perform Joseph-form stabilized covariance and state update.
    #[allow(clippy::needless_range_loop)]
    pub fn update(&mut self, det: &Detection) -> Result<(), TrackerError> {
        let r_var = (det.ce90_m / 2.146).powi(2).max(1e-6);
        let (s, det_s) = self.compute_s_matrix(r_var);
        let s_inv = Self::invert_s(s, det_s).ok_or(TrackerError::NonFiniteInput)?;
        let k = self.compute_kalman_gain(s_inv);
        let y = [det.east_m - self.x[0], det.north_m - self.x[1]];
        for i in 0..4 {
            self.x[i] += k[i][0] * y[0] + k[i][1] * y[1];
        }
        self.apply_joseph_update(k, r_var);
        self.hae_m = 0.8 * self.hae_m + 0.2 * det.hae_m;
        self.bbox = det.bbox;
        self.embedding = normalize_embedding(&det.embedding);
        self.hits += 1;
        self.time_since_update = 0;
        Ok(())
    }
}

/// Multi-target 3D tracker managing track lifecycle and data association.
#[derive(Debug, Clone)]
pub struct MultiTargetTracker {
    next_id: u64,
    pub tracks: Vec<Track>,
    pub min_hits: usize,
    pub max_age: usize,
    pub q_acc: f64,
}

impl Default for MultiTargetTracker {
    fn default() -> Self {
        Self::new(3, 5, 2.0)
    }
}

impl MultiTargetTracker {
    /// Create a new MultiTargetTracker with tracking parameters.
    pub fn new(min_hits: usize, max_age: usize, q_acc: f64) -> Self {
        Self {
            next_id: 1,
            tracks: Vec::new(),
            min_hits,
            max_age,
            q_acc,
        }
    }
}

impl MultiTargetTracker {
    fn score_candidate(trk: &Track, det: &Detection) -> Option<f64> {
        let d2 = match trk.mahalanobis_d2(det) {
            Some(v) if v <= 5.991 => v,
            _ => return None,
        };
        let (iou_val, cos_dist) = (
            trk.bbox.iou(&det.bbox),
            cosine_distance(&trk.embedding, &det.embedding),
        );
        if iou_val >= 0.1 || cos_dist <= 0.4 {
            Some(d2 + cos_dist)
        } else {
            None
        }
    }
}

impl MultiTargetTracker {
    fn find_best_detection(
        trk: &Track,
        detections: &[Detection],
        matched_dets: &[bool],
    ) -> Option<(usize, f64)> {
        let mut best: Option<(usize, f64)> = None;
        for (d_idx, det) in detections.iter().enumerate() {
            if matched_dets[d_idx] {
                continue;
            }
            if let Some(cost) = Self::score_candidate(trk, det) {
                if best.is_none() || cost < best.unwrap().1 {
                    best = Some((d_idx, cost));
                }
            }
        }
        best
    }
}

impl MultiTargetTracker {
    /// Associate detections with existing tracks using gated minimum cost.
    fn associate(&self, detections: &[Detection]) -> (Vec<(usize, usize)>, Vec<usize>, Vec<usize>) {
        let mut candidates = Vec::new();
        for (t_idx, trk) in self.tracks.iter().enumerate() {
            for (d_idx, det) in detections.iter().enumerate() {
                if let Some(cost) = Self::score_candidate(trk, det) {
                    candidates.push((cost, t_idx, d_idx));
                }
            }
        }
        candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let mut matches = Vec::new();
        let mut matched_tracks = vec![false; self.tracks.len()];
        let mut matched_dets = vec![false; detections.len()];
        for (_, t_idx, d_idx) in candidates {
            if !matched_tracks[t_idx] && !matched_dets[d_idx] {
                matches.push((t_idx, d_idx));
                matched_tracks[t_idx] = true;
                matched_dets[d_idx] = true;
            }
        }
        let unmatched_tracks = (0..self.tracks.len())
            .filter(|&i| !matched_tracks[i])
            .collect();
        let unmatched_dets = (0..detections.len())
            .filter(|&i| !matched_dets[i])
            .collect();
        (matches, unmatched_tracks, unmatched_dets)
    }
}

impl MultiTargetTracker {
    fn validate_step_inputs(
        &self,
        dt_s: f64,
        detections: &[Detection],
    ) -> Result<(), TrackerError> {
        if !dt_s.is_finite() || dt_s <= 0.0 || !self.q_acc.is_finite() || self.q_acc < 0.0 {
            return Err(TrackerError::InvalidDeltaTime);
        }
        for d in detections {
            if !d.east_m.is_finite()
                || !d.north_m.is_finite()
                || !d.hae_m.is_finite()
                || !d.ce90_m.is_finite()
                || !d.le90_m.is_finite()
                || d.ce90_m < 0.0
                || d.le90_m < 0.0
                || !d.bbox.x1.is_finite()
                || !d.bbox.y1.is_finite()
                || !d.bbox.x2.is_finite()
                || !d.bbox.y2.is_finite()
                || !d.embedding.iter().all(|e| e.is_finite())
            {
                return Err(TrackerError::NonFiniteInput);
            }
        }
        Ok(())
    }
}

impl MultiTargetTracker {
    fn update_tracks(
        &mut self,
        matches: Vec<(usize, usize)>,
        unmatched: Vec<usize>,
        dets: &[Detection],
    ) -> Result<(), TrackerError> {
        for (t_idx, d_idx) in matches {
            self.tracks[t_idx].update(&dets[d_idx])?;
            if self.tracks[t_idx].hits >= self.min_hits {
                self.tracks[t_idx].status = TrackStatus::Confirmed;
            } else {
                self.tracks[t_idx].status = TrackStatus::Tentative;
            }
        }
        for t_idx in unmatched {
            if self.tracks[t_idx].time_since_update > self.max_age {
                self.tracks[t_idx].status = TrackStatus::Deleted;
            } else {
                self.tracks[t_idx].status = TrackStatus::Coasting;
            }
        }
        Ok(())
    }
}

impl MultiTargetTracker {
    /// Execute one prediction, association, update, and pruning cycle.
    pub fn step(&mut self, dt_s: f64, detections: &[Detection]) -> Result<(), TrackerError> {
        self.validate_step_inputs(dt_s, detections)?;
        for trk in &mut self.tracks {
            trk.predict(dt_s, self.q_acc);
        }
        let (matches, unmatched_tracks, unmatched_dets) = self.associate(detections);
        self.update_tracks(matches, unmatched_tracks, detections)?;
        for d_idx in unmatched_dets {
            let trk = Track::new(self.next_id, &detections[d_idx]);
            self.next_id += 1;
            self.tracks.push(trk);
        }
        self.tracks.retain(|t| t.status != TrackStatus::Deleted);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_det(x: f64, y: f64, emb: [f32; 8]) -> Detection {
        Detection {
            east_m: x,
            north_m: y,
            hae_m: 10.0,
            ce90_m: 1.5,
            le90_m: 1.0,
            bbox: BoundingBox {
                x1: 10.0,
                y1: 10.0,
                x2: 50.0,
                y2: 50.0,
            },
            embedding: emb,
        }
    }

    #[test]
    fn test_iou_exactness() {
        let b1 = BoundingBox {
            x1: 0.0,
            y1: 0.0,
            x2: 10.0,
            y2: 10.0,
        };
        let b2 = BoundingBox {
            x1: 5.0,
            y1: 0.0,
            x2: 15.0,
            y2: 10.0,
        };
        assert!((b1.iou(&b2) - (50.0 / 150.0)).abs() < 1e-9);
        assert!((b1.iou(&b1) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_cosine_distance() {
        let e1 = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let e2 = [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert!((cosine_distance(&e1, &e1) - 0.0).abs() < 1e-6);
        assert!((cosine_distance(&e1, &e2) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_velocity_convergence_on_linear_trajectory() {
        let mut tracker = MultiTargetTracker::new(2, 3, 0.5);
        let emb = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        for i in 0..10 {
            let det = dummy_det(i as f64 * 5.0, i as f64 * 10.0, emb);
            tracker.step(1.0, &[det]).unwrap();
        }
        assert_eq!(tracker.tracks.len(), 1);
        let trk = &tracker.tracks[0];
        assert_eq!(trk.status, TrackStatus::Confirmed);
        assert!((trk.x[2] - 5.0).abs() < 0.5);
        assert!((trk.x[3] - 10.0).abs() < 0.5);
    }

    #[test]
    fn test_mahalanobis_outlier_rejection() {
        let mut tracker = MultiTargetTracker::new(2, 3, 0.5);
        let emb = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        tracker.step(1.0, &[dummy_det(0.0, 0.0, emb)]).unwrap();
        tracker.step(1.0, &[dummy_det(1.0, 1.0, emb)]).unwrap();
        let outlier = dummy_det(500.0, 500.0, emb);
        tracker.step(1.0, &[outlier]).unwrap();
        assert_eq!(tracker.tracks.len(), 2);
    }

    #[test]
    fn test_reid_coasting_recovery() {
        let mut tracker = MultiTargetTracker::new(2, 4, 0.5);
        let emb = [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        tracker.step(1.0, &[dummy_det(0.0, 0.0, emb)]).unwrap();
        tracker.step(1.0, &[dummy_det(5.0, 5.0, emb)]).unwrap();
        tracker.step(1.0, &[]).unwrap();
        assert_eq!(tracker.tracks[0].status, TrackStatus::Coasting);
        let mut det_jump = dummy_det(15.0, 15.0, emb);
        det_jump.bbox = BoundingBox {
            x1: 200.0,
            y1: 200.0,
            x2: 240.0,
            y2: 240.0,
        };
        tracker.step(1.0, &[det_jump]).unwrap();
        assert_eq!(tracker.tracks.len(), 1);
        assert_eq!(tracker.tracks[0].status, TrackStatus::Confirmed);
    }
}
