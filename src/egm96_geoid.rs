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

impl std::fmt::Display for Egm96Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteCoordinate => write!(f, "coordinates must be finite"),
            Self::LatitudeOutOfRange(lat) => {
                write!(f, "latitude {lat} out of range [-90, 90]")
            }
            Self::InvalidDimensions { cols, rows } => {
                write!(f, "grid dimensions must be >= 2x2, got {cols}x{rows}")
            }
            Self::InvalidStep => write!(f, "grid step must be positive and finite"),
            Self::DataLengthMismatch { expected, found } => {
                write!(f, "expected {expected} values, found {found}")
            }
        }
    }
}

impl std::error::Error for Egm96Error {}
