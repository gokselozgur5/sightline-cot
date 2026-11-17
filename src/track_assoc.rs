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
