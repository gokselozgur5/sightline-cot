//! EGM96 geoid height undulation grid interpolation.

/// Error conditions in EGM96 geoid grid operations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Egm96Error {
    /// Coordinate contains NaN or Infinity.
    NonFiniteCoordinate,
    /// Latitude lies outside [-90.0, 90.0] degrees.
    LatitudeOutOfRange(f64),
    /// Grid dimensions must have at least 2 rows and 2 columns.
    InvalidDimensions { cols: usize, rows: usize },
    /// Grid step size must be positive and finite.
    InvalidStep,
    /// Undulation buffer size does not match rows * cols.
    DataLengthMismatch { expected: usize, found: usize },
}
