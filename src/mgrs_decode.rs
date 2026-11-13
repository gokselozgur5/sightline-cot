//! MGRS 10-digit grid reference decoding to UTM and WGS84.

use crate::mgrs::GridError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MgrsDecodeError {
    InvalidLength,
    InvalidZone,
    InvalidBand,
    InvalidSquare,
    InvalidDigits,
    Grid(GridError),
}

impl std::fmt::Display for MgrsDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "MGRS string is too short"),
            Self::InvalidZone => write!(f, "invalid UTM zone (1..=60)"),
            Self::InvalidBand => write!(f, "invalid MGRS latitude band letter"),
            Self::InvalidSquare => write!(f, "invalid 100k grid square identifier"),
            Self::InvalidDigits => write!(f, "invalid or odd number of coordinate digits"),
            Self::Grid(e) => write!(f, "grid error: {e:?}"),
        }
    }
}

impl std::error::Error for MgrsDecodeError {}
