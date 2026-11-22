//! MGRS 10-digit grid reference decoding to UTM and WGS84.

use crate::mgrs::{GridError, UtmCoord};
use crate::mgrs_parse::{parse_mgrs, MgrsParseError};

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

impl From<MgrsParseError> for MgrsDecodeError {
    fn from(err: MgrsParseError) -> Self {
        match err {
            MgrsParseError::EmptyInput | MgrsParseError::InvalidLength => Self::InvalidLength,
            MgrsParseError::InvalidZone => Self::InvalidZone,
            MgrsParseError::InvalidBand => Self::InvalidBand,
            MgrsParseError::InvalidSquare => Self::InvalidSquare,
            MgrsParseError::InvalidDigits => Self::InvalidDigits,
            MgrsParseError::Grid(e) => Self::Grid(e),
        }
    }
}

/// Decode an MGRS grid string into a UTM coordinate.
pub fn decode_mgrs_utm(s: &str) -> Result<UtmCoord, MgrsDecodeError> {
    let parsed = parse_mgrs(s)?;
    parsed.to_utm().map_err(Into::into)
}

/// Decode an MGRS grid string to WGS84 latitude and longitude in degrees.
pub fn decode_mgrs(s: &str) -> Result<(f64, f64), MgrsDecodeError> {
    let parsed = parse_mgrs(s)?;
    parsed.to_wgs84().map_err(Into::into)
}
