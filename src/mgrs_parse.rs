//! MGRS string parser and conversion to WGS84 and UTM.

use crate::mgrs::{utm_to_wgs84, GridError, UtmCoord, MGRS_BANDS, MGRS_COLS, MGRS_ROWS};

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
                self.zone,
                self.band,
                self.col,
                self.row,
                self.easting,
                self.northing,
                width = p
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
                self.zone,
                self.band,
                self.col,
                self.row,
                self.easting,
                self.northing,
                width = p
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

fn decode_100k_col(zone: u8, col: u8) -> Result<f64, MgrsParseError> {
    let c_idx = MGRS_COLS
        .iter()
        .position(|&c| c == col)
        .ok_or(MgrsParseError::InvalidSquare)?;
    let base_idx = [0, 8, 16][((zone - 1) % 3) as usize];
    let col_idx = (c_idx + 24 - base_idx) % 24;
    Ok((col_idx + 1) as f64 * 100_000.0)
}

fn decode_100k_row(zone: u8, row: u8) -> Result<f64, MgrsParseError> {
    let r_idx = MGRS_ROWS
        .iter()
        .position(|&r| r == row)
        .ok_or(MgrsParseError::InvalidSquare)?;
    let row_offset = if (zone % 2) == 1 { 0 } else { 5 };
    let row_idx = (r_idx + 20 - row_offset) % 20;
    Ok(row_idx as f64 * 100_000.0)
}

fn band_approx_northing(band: char) -> Result<(char, f64), MgrsParseError> {
    let b_pos = MGRS_BANDS
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
    if !digits.chars().all(|c| c.is_ascii_digit()) || digits.len() & 1 != 0 || digits.len() > 10 {
        return Err(MgrsParseError::InvalidDigits);
    }
    Ok((prefix.to_string(), digits.to_string()))
}

fn parse_prefix(prefix: &str) -> Result<(u8, char, char, char), MgrsParseError> {
    let p = prefix.trim();
    let num_digits = p.chars().take_while(|c| c.is_ascii_digit()).count();
    if !(1..=2).contains(&num_digits) || p.len() != num_digits + 3 {
        return Err(MgrsParseError::InvalidLength);
    }
    let zone: u8 = p[..num_digits]
        .parse()
        .map_err(|_| MgrsParseError::InvalidZone)?;
    if !(1..=60).contains(&zone) {
        return Err(MgrsParseError::InvalidZone);
    }
    let mut chars = p[num_digits..].chars().map(|c| c.to_ascii_uppercase());
    let band = chars.next().ok_or(MgrsParseError::InvalidBand)?;
    let col = chars.next().ok_or(MgrsParseError::InvalidSquare)?;
    let row = chars.next().ok_or(MgrsParseError::InvalidSquare)?;
    if !MGRS_BANDS.contains(&(band as u8)) {
        return Err(MgrsParseError::InvalidBand);
    }
    if !MGRS_COLS.contains(&(col as u8)) || !MGRS_ROWS.contains(&(row as u8)) {
        return Err(MgrsParseError::InvalidSquare);
    }
    Ok((zone, band, col, row))
}

fn parse_numeric_digits(digits: &str) -> Result<(u32, u32, usize), MgrsParseError> {
    if digits.is_empty() {
        return Ok((0, 0, 0));
    }
    let p = digits.len() / 2;
    let easting: u32 = digits[..p]
        .parse()
        .map_err(|_| MgrsParseError::InvalidDigits)?;
    let northing: u32 = digits[p..]
        .parse()
        .map_err(|_| MgrsParseError::InvalidDigits)?;
    Ok((easting, northing, p))
}

/// Parse any standard formatted or compact MGRS string into a [`ParsedMgrs`].
pub fn parse_mgrs(s: &str) -> Result<ParsedMgrs, MgrsParseError> {
    let (prefix, digits) = extract_parts(s)?;
    let (zone, band, col, row) = parse_prefix(&prefix)?;
    let (easting, northing, precision) = parse_numeric_digits(&digits)?;
    Ok(ParsedMgrs {
        zone,
        band,
        col,
        row,
        easting,
        northing,
        precision,
    })
}

fn cell_center_offset(precision: usize, easting: u32, northing: u32) -> (f64, f64) {
    if precision == 0 {
        (50_000.0, 50_000.0)
    } else {
        let scale = 10_f64.powi(5 - precision as i32);
        (
            easting as f64 * scale + scale * 0.5,
            northing as f64 * scale + scale * 0.5,
        )
    }
}

fn resolve_mgrs_northing(base_n: f64, approx_n: f64) -> f64 {
    let mut best_n = base_n;
    let mut best_diff = (base_n - approx_n).abs();
    for k in 1..=5 {
        let cand = k as f64 * 2_000_000.0 + base_n;
        let diff = (cand - approx_n).abs();
        if diff < best_diff {
            best_diff = diff;
            best_n = cand;
        }
    }
    best_n
}

impl ParsedMgrs {
    /// Convert parsed MGRS coordinate to a UTM coordinate centered in the grid cell.
    pub fn to_utm(&self) -> Result<UtmCoord, MgrsParseError> {
        let (hemi, approx_n) = band_approx_northing(self.band)?;
        let col_e = decode_100k_col(self.zone, self.col as u8)?;
        let row_n = decode_100k_row(self.zone, self.row as u8)?;
        let (rem_e, rem_n) = cell_center_offset(self.precision, self.easting, self.northing);
        let northing = resolve_mgrs_northing(row_n + rem_n, approx_n);
        Ok(UtmCoord {
            zone: self.zone,
            hemisphere: hemi,
            easting: col_e + rem_e,
            northing,
        })
    }
}

impl ParsedMgrs {
    /// Convert parsed MGRS coordinate to WGS84 (latitude, longitude) in degrees.
    pub fn to_wgs84(&self) -> Result<(f64, f64), MgrsParseError> {
        let utm = self.to_utm()?;
        utm_to_wgs84(&utm).map_err(MgrsParseError::Grid)
    }
}

/// Convenience function to directly parse an MGRS string into a UTM coordinate.
pub fn parse_mgrs_to_utm(s: &str) -> Result<UtmCoord, MgrsParseError> {
    parse_mgrs(s)?.to_utm()
}

/// Convenience function to directly parse an MGRS string into WGS84 (lat, lon) degrees.
pub fn parse_mgrs_to_wgs84(s: &str) -> Result<(f64, f64), MgrsParseError> {
    parse_mgrs(s)?.to_wgs84()
}
