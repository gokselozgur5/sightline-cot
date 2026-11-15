//! ASCII grid format loader for geoid undulation tables.

/// Error conditions during ASCII geoid grid parsing and sampling.
#[derive(Debug, Clone, PartialEq)]
pub enum GeoidError {
    EmptyInput,
    InvalidHeader(String),
    MissingHeader(&'static str),
    InvalidDimensions { cols: usize, rows: usize },
    InvalidCellSize(f64),
    InvalidDataValue(String),
    DataCountMismatch { expected: usize, found: usize },
}

impl std::fmt::Display for GeoidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "empty ascii grid input"),
            Self::InvalidHeader(m) => write!(f, "invalid ascii header: {}", m),
            Self::MissingHeader(h) => write!(f, "missing required header: {}", h),
            Self::InvalidDimensions { cols, rows } => {
                write!(f, "dimensions must be >= 1, got {}x{}", cols, rows)
            }
            Self::InvalidCellSize(s) => write!(f, "invalid cell size: {}", s),
            Self::InvalidDataValue(v) => write!(f, "invalid float token: {}", v),
            Self::DataCountMismatch { expected, found } => {
                write!(f, "expected {} values, found {}", expected, found)
            }
        }
    }
}

impl std::error::Error for GeoidError {}

/// Regular geoid undulation grid loaded from an ASCII raster.
#[derive(Debug, Clone, PartialEq)]
pub struct GeoidGrid {
    pub cols: usize,
    pub rows: usize,
    pub xllcorner: f64,
    pub yllcorner: f64,
    pub cellsize: f64,
    pub nodata_value: Option<f64>,
    pub data: Vec<Option<f64>>,
}

impl GeoidGrid {
    /// Construct and validate a new GeoidGrid raster instance.
    pub fn new(
        cols: usize, rows: usize, x0: f64, y0: f64, step: f64,
        nodata: Option<f64>, data: Vec<Option<f64>>,
    ) -> Result<Self, GeoidError> {
        if cols == 0 || rows == 0 {
            return Err(GeoidError::InvalidDimensions { cols, rows });
        }
        if !step.is_finite() || step <= 0.0 {
            return Err(GeoidError::InvalidCellSize(step));
        }
        if !x0.is_finite() || !y0.is_finite() {
            return Err(GeoidError::InvalidHeader("non-finite origin".into()));
        }
        if data.len() != cols * rows {
            let (expected, found) = (cols * rows, data.len());
            return Err(GeoidError::DataCountMismatch { expected, found });
        }
        Ok(Self { cols, rows, xllcorner: x0, yllcorner: y0, cellsize: step, nodata_value: nodata, data })
    }
}
