//! Direct projection from UTM coordinates to MGRS grid strings.

use crate::mgrs::{lat_to_mgrs_band, utm_to_wgs84, GridError, MgrsString, UtmCoord};

const COLS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const ROWS: &[u8] = b"ABCDEFGHJKLMNPQRSTUV";

fn col_letter(zone: u8, easting: f64) -> char {
    let set_idx = ((zone - 1) % 3) as usize;
    let base_idx = [0, 8, 16][set_idx];
    let col_idx = ((easting / 100000.0).floor() as usize).saturating_sub(1);
    COLS[(base_idx + col_idx) % COLS.len()] as char
}

fn row_letter(zone: u8, northing: f64) -> char {
    let row_offset = if (zone % 2) == 1 { 0 } else { 5 };
    let row_idx = ((northing / 100000.0).floor() as usize) % 20;
    ROWS[(row_offset + row_idx) % ROWS.len()] as char
}
