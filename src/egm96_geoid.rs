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

/// Regular geodetic grid for EGM96 geoid undulation interpolation.
#[derive(Debug, Clone, PartialEq)]
pub struct Egm96Geoid {
    pub min_lat_deg: f64,
    pub max_lat_deg: f64,
    pub lat_step_deg: f64,
    pub lon_step_deg: f64,
    pub cols: usize,
    pub rows: usize,
    pub undulations: Vec<f64>,
}

impl Egm96Geoid {
    /// Validate input dimensions and grid step sizes.
    fn validate_grid(
        lat_step: f64,
        lon_step: f64,
        cols: usize,
        rows: usize,
        data_len: usize,
    ) -> Result<(), Egm96Error> {
        if !lat_step.is_finite() || lat_step <= 0.0 || !lon_step.is_finite() || lon_step <= 0.0 {
            return Err(Egm96Error::InvalidStep);
        }
        if cols < 2 || rows < 2 {
            return Err(Egm96Error::InvalidDimensions { cols, rows });
        }
        let expected = cols * rows;
        if data_len != expected {
            return Err(Egm96Error::DataLengthMismatch { expected, found: data_len });
        }
        Ok(())
    }
}
