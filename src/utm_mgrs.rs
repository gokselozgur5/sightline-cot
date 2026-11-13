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

fn append_digits(buf: &mut [u8], len: &mut usize, mut val: u32, digits: usize) {
    let mut temp = [b'0'; 10];
    for i in (0..digits).rev() {
        temp[i] = b'0' + (val % 10) as u8;
        val /= 10;
    }
    for b in &temp[..digits] {
        buf[*len] = *b;
        *len += 1;
    }
}

fn init_mgrs_header(zone: u8, band: char, col: char, row: char) -> MgrsString {
    let mut s = MgrsString {
        bytes: [0u8; 32],
        len: 0,
    };
    if zone < 10 {
        s.bytes[s.len] = b'0';
        s.len += 1;
    }
    append_digits(
        &mut s.bytes,
        &mut s.len,
        zone as u32,
        if zone < 10 { 1 } else { 2 },
    );
    s.bytes[s.len] = band as u8;
    s.bytes[s.len + 1] = col as u8;
    s.bytes[s.len + 2] = row as u8;
    s.len += 3;
    s
}
