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
