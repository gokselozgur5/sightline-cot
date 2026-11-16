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

/// Normalize longitude into standard [0.0, 360.0) range.
pub fn normalize_lon_360(lon_deg: f64) -> f64 {
    let rem = lon_deg % 360.0;
    if rem < 0.0 {
        rem + 360.0
    } else {
        rem
    }
}

/// Normalize longitude into standard [-180.0, 180.0) range.
pub fn normalize_lon_180(lon_deg: f64) -> f64 {
    let lon360 = normalize_lon_360(lon_deg);
    if lon360 >= 180.0 {
        lon360 - 360.0
    } else {
        lon360
    }
}
