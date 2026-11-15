//! MGRS string parser and conversion to WGS84 and UTM.

use crate::mgrs::{utm_to_wgs84, GridError, UtmCoord};

/// Error conditions encountered during MGRS string parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MgrsParseError {
    EmptyInput,
    InvalidLength,
    InvalidZone,
    InvalidBand,
    InvalidSquare,
    InvalidDigits,
    Grid(GridError),
}

impl core::fmt::Display for MgrsParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "empty MGRS string"),
            Self::InvalidLength => write!(f, "invalid MGRS length"),
            Self::InvalidZone => write!(f, "invalid UTM zone (1..=60)"),
            Self::InvalidBand => write!(f, "invalid latitude band letter"),
            Self::InvalidSquare => write!(f, "invalid 100k grid square"),
            Self::InvalidDigits => write!(f, "invalid coordinate digits"),
            Self::Grid(e) => write!(f, "grid error: {e:?}"),
        }
    }
}

impl std::error::Error for MgrsParseError {}

impl From<GridError> for MgrsParseError {
    fn from(err: GridError) -> Self {
        Self::Grid(err)
    }
}

/// Parsed MGRS coordinate representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedMgrs {
    pub zone: u8,
    pub band: char,
    pub col: char,
    pub row: char,
    pub easting: u32,
    pub northing: u32,
    pub precision: usize,
}
