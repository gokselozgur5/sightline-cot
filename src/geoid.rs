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
