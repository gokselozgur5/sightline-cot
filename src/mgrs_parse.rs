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

impl ParsedMgrs {
    /// Return the grid cell resolution in meters.
    pub fn resolution_meters(&self) -> f64 {
        match self.precision {
            0 => 100_000.0,
            1 => 10_000.0,
            2 => 1_000.0,
            3 => 100.0,
            4 => 10.0,
            _ => 1.0,
        }
    }

    /// Format as compact MGRS string.
    pub fn format_compact(&self) -> String {
        if self.precision == 0 {
            format!("{:02}{}{}{}", self.zone, self.band, self.col, self.row)
        } else {
            let p = self.precision;
            format!(
                "{:02}{}{}{}{:0width$}{:0width$}",
                self.zone, self.band, self.col, self.row, self.easting, self.northing, width = p
            )
        }
    }
}

impl ParsedMgrs {
    /// Format as spaced MGRS string.
    pub fn format_spaced(&self) -> String {
        if self.precision == 0 {
            format!("{:02}{} {}{}", self.zone, self.band, self.col, self.row)
        } else {
            let p = self.precision;
            format!(
                "{:02}{} {}{} {:0width$} {:0width$}",
                self.zone, self.band, self.col, self.row, self.easting, self.northing, width = p
            )
        }
    }
}

impl core::fmt::Display for ParsedMgrs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.format_compact())
    }
}

impl core::str::FromStr for ParsedMgrs {
    type Err = MgrsParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_mgrs(s)
    }
}
