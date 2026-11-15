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
