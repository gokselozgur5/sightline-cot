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

const COLS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const ROWS: &[u8] = b"ABCDEFGHJKLMNPQRSTUV";

fn decode_100k_col(zone: u8, col: u8) -> Result<f64, MgrsParseError> {
    let c_idx = COLS
        .iter()
        .position(|&c| c == col)
        .ok_or(MgrsParseError::InvalidSquare)?;
    let base_idx = [0, 8, 16][((zone - 1) % 3) as usize];
    let col_idx = (c_idx + 24 - base_idx) % 24;
    Ok((col_idx + 1) as f64 * 100_000.0)
}

fn decode_100k_row(zone: u8, row: u8) -> Result<f64, MgrsParseError> {
    let r_idx = ROWS
        .iter()
        .position(|&r| r == row)
        .ok_or(MgrsParseError::InvalidSquare)?;
    let row_offset = if (zone % 2) == 1 { 0 } else { 5 };
    let row_idx = (r_idx + 20 - row_offset) % 20;
    Ok(row_idx as f64 * 100_000.0)
}

const BANDS: &[u8] = b"CDEFGHJKLMNPQRSTUVWX";

fn band_approx_northing(band: char) -> Result<(char, f64), MgrsParseError> {
    let b_pos = BANDS
        .iter()
        .position(|&b| b == band as u8)
        .ok_or(MgrsParseError::InvalidBand)?;
    let hemi = if band >= 'N' { 'N' } else { 'S' };
    let lat = if b_pos == 19 {
        78.0
    } else {
        -80.0 + (b_pos as f64) * 8.0 + 4.0
    };
    let n = if hemi == 'N' {
        lat * 111_132.0
    } else {
        10_000_000.0 + lat * 111_132.0
    };
    Ok((hemi, n))
}

fn extract_parts(s: &str) -> Result<(String, String), MgrsParseError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(MgrsParseError::EmptyInput);
    }
    if !s.is_ascii() {
        return Err(MgrsParseError::InvalidSquare);
    }
    let tokens: Vec<&str> = s.split_whitespace().collect();
    if tokens.len() >= 2 {
        let last = tokens[tokens.len() - 1];
        let prev = tokens[tokens.len() - 2];
        if last.chars().all(|c| c.is_ascii_digit()) && prev.chars().all(|c| c.is_ascii_digit()) {
            if last.len() != prev.len() || last.len() > 5 {
                return Err(MgrsParseError::InvalidDigits);
            }
            let prefix = tokens[..tokens.len() - 2].concat();
            let digits = format!("{prev}{last}");
            return Ok((prefix, digits));
        }
    }
    let compact: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    split_compact(&compact)
}

fn split_compact(s: &str) -> Result<(String, String), MgrsParseError> {
    let b = s.as_bytes();
    let zone_len = if b.len() >= 2 && b[0].is_ascii_digit() && b[1].is_ascii_digit() {
        2
    } else if !b.is_empty() && b[0].is_ascii_digit() {
        1
    } else {
        return Err(MgrsParseError::InvalidZone);
    };
    if b.len() < zone_len + 3 {
        return Err(MgrsParseError::InvalidLength);
    }
    let prefix = &s[..zone_len + 3];
    let digits = &s[zone_len + 3..];
    if !digits.chars().all(|c| c.is_ascii_digit()) || digits.len() % 2 != 0 || digits.len() > 10 {
        return Err(MgrsParseError::InvalidDigits);
    }
    Ok((prefix.to_string(), digits.to_string()))
}
