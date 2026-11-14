//! MGRS precision truncation module.
use crate::mgrs::GridError;

/// Target precision level for MGRS truncation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MgrsPrecision {
    OneMeter,
    TenMeters,
    HundredMeters,
    OneKilometer,
    TenKilometers,
}

impl MgrsPrecision {
    /// Number of digits per coordinate axis (easting and northing).
    pub fn digits_per_axis(&self) -> usize {
        match self {
            Self::OneMeter => 5,
            Self::TenMeters => 4,
            Self::HundredMeters => 3,
            Self::OneKilometer => 2,
            Self::TenKilometers => 1,
        }
    }
}

fn validate_mgrs_10digit(s: &str) -> Result<(&str, &str, &str), GridError> {
    if s.len() != 15 || !s.is_ascii() {
        return Err(GridError::InvalidZone);
    }
    let b = s.as_bytes();
    let zone: u8 = s[0..2].parse().map_err(|_| GridError::InvalidZone)?;
    if !(1..=60).contains(&zone) {
        return Err(GridError::InvalidZone);
    }
    if !b[2].is_ascii_uppercase() || !b[3].is_ascii_uppercase() || !b[4].is_ascii_uppercase() {
        return Err(GridError::InvalidLatitude);
    }
    if !b[5..15].iter().all(|c| c.is_ascii_digit()) {
        return Err(GridError::InvalidEasting);
    }
    Ok((&s[0..5], &s[5..10], &s[10..15]))
}

/// Truncate a 10-digit MGRS string to the given target precision.
pub fn truncate_mgrs(mgrs: &str, precision: MgrsPrecision) -> Result<String, GridError> {
    let (prefix, easting, northing) = validate_mgrs_10digit(mgrs)?;
    let d = precision.digits_per_axis();
    let mut out = String::with_capacity(5 + 2 * d);
    out.push_str(prefix);
    out.push_str(&easting[..d]);
    out.push_str(&northing[..d]);
    Ok(out)
}
