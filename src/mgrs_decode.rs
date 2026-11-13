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

const COLS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const ROWS: &[u8] = b"ABCDEFGHJKLMNPQRSTUV";

fn decode_100k_col(zone: u8, col: u8) -> Result<f64, MgrsDecodeError> {
    let c_idx = COLS
        .iter()
        .position(|&c| c == col)
        .ok_or(MgrsDecodeError::InvalidSquare)?;
    let base_idx = [0, 8, 16][((zone - 1) % 3) as usize];
    let col_idx = (c_idx + 24 - base_idx) % 24;
    Ok((col_idx + 1) as f64 * 100_000.0)
}

fn decode_100k_row(zone: u8, row: u8) -> Result<f64, MgrsDecodeError> {
    let r_idx = ROWS
        .iter()
        .position(|&r| r == row)
        .ok_or(MgrsDecodeError::InvalidSquare)?;
    let row_offset = if (zone % 2) == 1 { 0 } else { 5 };
    let row_idx = (r_idx + 20 - row_offset) % 20;
    Ok(row_idx as f64 * 100_000.0)
}
