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
