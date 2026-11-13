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

/// Project a UTM coordinate and latitude band directly to an MGRS grid string.
pub fn utm_to_mgrs_with_band(
    utm: &UtmCoord,
    band: char,
    digits: usize,
) -> Result<MgrsString, GridError> {
    if !(1..=5).contains(&digits) {
        return Err(GridError::InvalidNorthing);
    }
    if !(1..=60).contains(&utm.zone) {
        return Err(GridError::InvalidZone);
    }
    let col = col_letter(utm.zone, utm.easting);
    let row = row_letter(utm.zone, utm.northing);
    let mut s = init_mgrs_header(utm.zone, band, col, row);
    let div = 100_000.0 / (10u32.pow(digits as u32) as f64);
    let e = ((utm.easting % 100000.0) / div).floor() as u32;
    let n = ((utm.northing % 100000.0) / div).floor() as u32;
    append_digits(&mut s.bytes, &mut s.len, e, digits);
    append_digits(&mut s.bytes, &mut s.len, n, digits);
    Ok(s)
}

/// Project a UTM coordinate directly to an MGRS grid string using its implicit band.
pub fn utm_to_mgrs(utm: &UtmCoord, digits: usize) -> Result<MgrsString, GridError> {
    let (lat, _) = utm_to_wgs84(utm)?;
    let band = lat_to_mgrs_band(lat)?;
    utm_to_mgrs_with_band(utm, band, digits)
}
