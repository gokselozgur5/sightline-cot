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
