//! Geoid undulation models and height conversions.

/// Error conditions in geoid grid configuration and evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeoidError {
    InvalidDimensions,
    InvalidStep,
    InvalidBounds,
    NonFiniteCoordinate,
    DataLengthMismatch,
}

impl std::fmt::Display for GeoidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDimensions => write!(f, "grid dimensions must be at least 2x2"),
            Self::InvalidStep => write!(f, "step sizes must be positive and finite"),
            Self::InvalidBounds => write!(f, "grid bounds must lie within valid geodetic range"),
            Self::NonFiniteCoordinate => write!(f, "coordinates must be finite"),
            Self::DataLengthMismatch => write!(f, "undulation count must match cols * rows"),
        }
    }
}

impl std::error::Error for GeoidError {}
