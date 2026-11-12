//! NATO Military Grid Reference System (MGRS) & UTM conversions.

/// Error conditions in UTM and MGRS transformations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridError {
    InvalidLatitude,
    InvalidLongitude,
    InvalidZone,
    InvalidNorthing,
    InvalidEasting,
    BufferTooSmall,
}

/// UTM coordinate representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UtmCoord {
    pub zone: u8,
    pub hemisphere: char,
    pub easting: f64,
    pub northing: f64,
}

const WGS84_A: f64 = 6378137.0;
const WGS84_F: f64 = 1.0 / 298.257223563;
const UTM_K0: f64 = 0.9996;
const FALSE_EASTING: f64 = 500000.0;
const FALSE_NORTHING_SOUTH: f64 = 10000000.0;

/// Determine standard UTM zone from longitude in degrees.
pub fn lon_to_utm_zone(lon_deg: f64) -> Result<u8, GridError> {
    if !(-180.0..=180.0).contains(&lon_deg) {
        return Err(GridError::InvalidLongitude);
    }
    let zone = ((lon_deg + 180.0) / 6.0).floor() as u8 + 1;
    Ok(zone.clamp(1, 60))
}

/// Forward UTM projection from WGS84 lat/lon.
pub fn wgs84_to_utm(lat_deg: f64, lon_deg: f64) -> Result<UtmCoord, GridError> {
    if !(-80.0..=84.0).contains(&lat_deg) {
        return Err(GridError::InvalidLatitude);
    }
    let zone = lon_to_utm_zone(lon_deg)?;
    let hemisphere = if lat_deg >= 0.0 { 'N' } else { 'S' };
    let (east, north) = utm_project_point(lat_deg, lon_deg, zone, hemisphere);
    Ok(UtmCoord {
        zone,
        hemisphere,
        easting: east,
        northing: north,
    })
}

fn utm_project_point(lat_deg: f64, lon_deg: f64, zone: u8, hemi: char) -> (f64, f64) {
    let lat = lat_deg.to_radians();
    let lon0 = ((zone as f64 - 1.0) * 6.0 - 180.0 + 3.0).to_radians();
    let dlon = lon_deg.to_radians() - lon0;
    let (sin_lat, cos_lat) = (lat.sin(), lat.cos());
    let tan_lat = lat.tan();
    let e2 = WGS84_F * (2.0 - WGS84_F);
    let ep2 = e2 / (1.0 - e2);
    let nu = WGS84_A / (1.0 - e2 * sin_lat * sin_lat).sqrt();
    let m = meridian_arc(lat, e2);
    let eta2 = ep2 * cos_lat * cos_lat;
    let t = tan_lat * tan_lat;
    let p = dlon * cos_lat;
    let p2 = p * p;
    let p3 = p2 * p;
    let p4 = p2 * p2;
    let p5 = p4 * p;
    let east = FALSE_EASTING
        + UTM_K0
            * nu
            * (p + (1.0 - t + eta2) * p3 / 6.0
                + (5.0 - 18.0 * t + t * t + 72.0 * eta2) * p5 / 120.0);
    let north_val = UTM_K0
        * (m + nu * tan_lat * (p2 / 2.0 + (5.0 - t + 9.0 * eta2 + 4.0 * eta2 * eta2) * p4 / 24.0));
    let north = if hemi == 'S' {
        FALSE_NORTHING_SOUTH + north_val
    } else {
        north_val
    };
    (east, north)
}

fn meridian_arc(phi: f64, e2: f64) -> f64 {
    let e4 = e2 * e2;
    let e6 = e4 * e2;
    let a0 = 1.0 - e2 / 4.0 - 3.0 * e4 / 64.0 - 5.0 * e6 / 256.0;
    let a2 = 3.0 * e2 / 8.0 + 3.0 * e4 / 32.0 + 45.0 * e6 / 1024.0;
    let a4 = 15.0 * e4 / 256.0 + 45.0 * e6 / 1024.0;
    let a6 = 35.0 * e6 / 3072.0;
    WGS84_A * (a0 * phi - a2 * (2.0 * phi).sin() + a4 * (4.0 * phi).sin() - a6 * (6.0 * phi).sin())
}

/// Reverse UTM projection from UTM to WGS84 lat/lon.
pub fn utm_to_wgs84(coord: &UtmCoord) -> Result<(f64, f64), GridError> {
    if !(1..=60).contains(&coord.zone) {
        return Err(GridError::InvalidZone);
    }
    if coord.hemisphere != 'N' && coord.hemisphere != 'S' {
        return Err(GridError::InvalidLatitude);
    }
    if !(100000.0..=900000.0).contains(&coord.easting) {
        return Err(GridError::InvalidEasting);
    }
    if !(0.0..=10000000.0).contains(&coord.northing) {
        return Err(GridError::InvalidNorthing);
    }
    Ok(utm_unproject_point(coord))
}

fn find_footpoint_latitude(m: f64, e2: f64) -> f64 {
    let mut phi = m / (WGS84_A * (1.0 - e2 / 4.0));
    for _ in 0..6 {
        let m_eval = meridian_arc(phi, e2);
        let d = (1.0 - e2 * phi.sin().powi(2)).sqrt();
        let dm = WGS84_A * (1.0 - e2) / (d * d * d);
        let diff = (m - m_eval) / dm;
        phi += diff;
        if diff.abs() < 1e-12 {
            break;
        }
    }
    phi
}

fn utm_unproject_point(coord: &UtmCoord) -> (f64, f64) {
    let e2 = WGS84_F * (2.0 - WGS84_F);
    let ep2 = e2 / (1.0 - e2);
    let y = if coord.hemisphere == 'S' {
        coord.northing - FALSE_NORTHING_SOUTH
    } else {
        coord.northing
    };
    let x = coord.easting - FALSE_EASTING;
    let m = y / UTM_K0;
    let phi1 = find_footpoint_latitude(m, e2);
    let (sin_phi1, cos_phi1) = (phi1.sin(), phi1.cos());
    let tan_phi1 = phi1.tan();
    let nu1 = WGS84_A / (1.0 - e2 * sin_phi1 * sin_phi1).sqrt();
    let rho1 = WGS84_A * (1.0 - e2) / (1.0 - e2 * sin_phi1 * sin_phi1).powf(1.5);
    let d = x / (nu1 * UTM_K0);
    let (d2, d3, d4, d5, d6) = (
        d * d,
        d * d * d,
        d * d * d * d,
        d * d * d * d * d,
        d * d * d * d * d * d,
    );
    let t1 = tan_phi1 * tan_phi1;
    let eta1_2 = ep2 * cos_phi1 * cos_phi1;
    let lat = phi1
        - (nu1 * tan_phi1 / rho1)
            * (d2 / 2.0
                - (5.0 + 3.0 * t1 + 10.0 * eta1_2 - 4.0 * eta1_2 * eta1_2 - 9.0 * ep2) * d4 / 24.0
                + (61.0 + 90.0 * t1 + 298.0 * eta1_2 + 45.0 * t1 * t1) * d6 / 720.0);
    let lon0 = ((coord.zone as f64 - 1.0) * 6.0 - 180.0 + 3.0).to_radians();
    let lon = lon0
        + (d - (1.0 + 2.0 * t1 + eta1_2) * d3 / 6.0
            + (5.0 - 2.0 * eta1_2 + 28.0 * t1 - 3.0 * eta1_2 * eta1_2
                + 8.0 * ep2
                + 24.0 * t1 * t1)
                * d5
                / 120.0)
            / cos_phi1;
    (lat.to_degrees(), lon.to_degrees())
}

/// Latitude band letter determination according to MGRS.
pub fn lat_to_mgrs_band(lat_deg: f64) -> Result<char, GridError> {
    if !(-80.0..=84.0).contains(&lat_deg) {
        return Err(GridError::InvalidLatitude);
    }
    const BANDS: &[u8] = b"CDEFGHJKLMNPQRSTUVWX";
    let idx = if lat_deg >= 80.0 {
        19
    } else {
        ((lat_deg + 80.0) / 8.0).floor() as usize
    };
    Ok(BANDS[idx.min(BANDS.len() - 1)] as char)
}

fn mgrs_100k_col_letter(zone: u8, easting: f64) -> char {
    const SET_ORIGIN: [usize; 3] = [0, 8, 16];
    let set_idx = ((zone - 1) % 3) as usize;
    let base_idx = SET_ORIGIN[set_idx];
    let col_idx = ((easting / 100000.0).floor() as usize).saturating_sub(1);
    const COLS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
    COLS[(base_idx + col_idx) % COLS.len()] as char
}

fn mgrs_100k_row_letter(zone: u8, northing: f64) -> char {
    let row_offset = if (zone % 2) == 1 { 0 } else { 5 };
    let row_idx = ((northing / 100000.0).floor() as usize) % 20;
    const ROWS: &[u8] = b"ABCDEFGHJKLMNPQRSTUV";
    ROWS[(row_offset + row_idx) % ROWS.len()] as char
}

/// Fixed stack buffer for zero-alloc MGRS string representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MgrsString {
    pub bytes: [u8; 32],
    pub len: usize,
}

impl MgrsString {
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("")
    }
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

/// Format MGRS coordinate string at specified precision (1m, 10m, 100m, 1000m).
pub fn format_mgrs(
    lat_deg: f64,
    lon_deg: f64,
    digits_per_axis: usize,
) -> Result<MgrsString, GridError> {
    if !(1..=5).contains(&digits_per_axis) {
        return Err(GridError::InvalidNorthing);
    }
    let utm = wgs84_to_utm(lat_deg, lon_deg)?;
    let band = lat_to_mgrs_band(lat_deg)?;
    let col = mgrs_100k_col_letter(utm.zone, utm.easting);
    let row = mgrs_100k_row_letter(utm.zone, utm.northing);
    let mut s = MgrsString {
        bytes: [0u8; 32],
        len: 0,
    };
    if utm.zone < 10 {
        s.bytes[s.len] = b'0';
        s.len += 1;
    }
    append_digits(
        &mut s.bytes,
        &mut s.len,
        utm.zone as u32,
        if utm.zone < 10 { 1 } else { 2 },
    );
    s.bytes[s.len] = band as u8;
    s.len += 1;
