//! Standard base32 geohash encoder and decoder with bounding box calculation.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeohashError {
    InvalidCharacter(char),
    InvalidPrecision,
    EmptyHash,
    LatitudeOutOfRange,
    LongitudeOutOfRange,
    AtPole,
}

#[allow(non_upper_case_globals)]
impl GeohashError {
    #[deprecated(note = "use EmptyHash")]
    pub const EmptyString: Self = Self::EmptyHash;
}

impl std::fmt::Display for GeohashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCharacter(c) => write!(f, "invalid character in geohash: '{c}'"),
            Self::InvalidPrecision => write!(f, "precision must be between 1 and 12"),
            Self::EmptyHash => write!(f, "geohash string cannot be empty"),
            Self::LatitudeOutOfRange => write!(f, "latitude must be within [-90.0, 90.0]"),
            Self::LongitudeOutOfRange => write!(f, "longitude must be within [-180.0, 180.0]"),
            Self::AtPole => write!(f, "cannot calculate polar neighbor beyond bounds"),
        }
    }
}

impl std::error::Error for GeohashError {}

pub use crate::geo_bbox::GeoBbox;

const BASE32_CHARS: &[u8; 32] = b"0123456789bcdefghjkmnpqrstuvwxyz";

fn char_to_value(c: char) -> Result<u8, GeohashError> {
    if !c.is_ascii() {
        return Err(GeohashError::InvalidCharacter(c));
    }
    let lower = c.to_ascii_lowercase();
    match BASE32_CHARS.iter().position(|&b| b == lower as u8) {
        Some(idx) => Ok(idx as u8),
        None => Err(GeohashError::InvalidCharacter(c)),
    }
}

fn validate_coords(lat: f64, lon: f64, precision: usize) -> Result<(), GeohashError> {
    if precision == 0 || precision > 12 {
        return Err(GeohashError::InvalidPrecision);
    }
    if !(-90.0..=90.0).contains(&lat) || lat.is_nan() {
        return Err(GeohashError::LatitudeOutOfRange);
    }
    if !(-180.0..=180.0).contains(&lon) || lon.is_nan() {
        return Err(GeohashError::LongitudeOutOfRange);
    }
    Ok(())
}

/// Encodes latitude and longitude into a geohash string with given precision.
pub fn encode(lat: f64, lon: f64, precision: usize) -> Result<String, GeohashError> {
    validate_coords(lat, lon, precision)?;
    let (mut lat_r, mut lon_r) = ([-90.0, 90.0], [-180.0, 180.0]);
    let (mut bits, mut bit_count, mut is_lon) = (0u8, 0, true);
    let mut out = String::with_capacity(precision);
    while out.len() < precision {
        let (r, val) = if is_lon {
            (&mut lon_r, lon)
        } else {
            (&mut lat_r, lat)
        };
        let mid = (r[0] + r[1]) * 0.5;
        let bit = u8::from(val >= mid);
        bits = (bits << 1) | bit;
        r[(1 - bit) as usize] = mid;
        is_lon = !is_lon;
        bit_count += 1;
        if bit_count == 5 {
            out.push(BASE32_CHARS[bits as usize] as char);
            bits = 0;
            bit_count = 0;
        }
    }
    Ok(out)
}

/// Alias for [`encode`].
pub fn encode_geohash(lat: f64, lon: f64, precision: usize) -> Result<String, GeohashError> {
    encode(lat, lon, precision)
}

fn decode_intervals(hash: &str) -> Result<([f64; 2], [f64; 2]), GeohashError> {
    if hash.is_empty() {
        return Err(GeohashError::EmptyHash);
    }
    let (mut lat_r, mut lon_r) = ([-90.0, 90.0], [-180.0, 180.0]);
    let mut is_lon = true;
    for c in hash.chars() {
        let val = char_to_value(c)?;
        for i in (0..5).rev() {
            let bit = (val >> i) & 1;
            let r = if is_lon { &mut lon_r } else { &mut lat_r };
            let mid = (r[0] + r[1]) * 0.5;
            r[(1 - bit) as usize] = mid;
            is_lon = !is_lon;
        }
    }
    Ok((lat_r, lon_r))
}

/// Decodes a geohash string into its bounding box bounds.
pub fn decode_bbox(hash: &str) -> Result<GeoBbox, GeohashError> {
    let (lat_r, lon_r) = decode_intervals(hash)?;
    Ok(GeoBbox::new(lat_r[0], lon_r[0], lat_r[1], lon_r[1]))
}

/// Alias for [`decode_bbox`].
pub fn decode_geohash_bbox(hash: &str) -> Result<GeoBbox, GeohashError> {
    decode_bbox(hash)
}

/// Decodes a geohash string into the center point `(latitude, longitude)`.
pub fn decode(hash: &str) -> Result<(f64, f64), GeohashError> {
    let bbox = decode_bbox(hash)?;
    let center = bbox.center();
    Ok((center[0], center[1]))
}

/// Alias for [`decode`].
pub fn decode_geohash(hash: &str) -> Result<(f64, f64), GeohashError> {
    decode(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_sanity() {
        assert_eq!(encode(0.0, 0.0, 1).unwrap(), "s");
        let (lat, lon) = decode("s").unwrap();
        assert_eq!(lat, 22.5);
        assert_eq!(lon, 22.5);
    }
}
