//! Degrees-minutes-seconds (DMS) coordinate parsing and formatting.

/// Errors that can occur during DMS coordinate parsing and formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmsError {
    EmptyInput,
    InvalidFormat,
    InvalidDegrees,
    InvalidMinutes,
    InvalidSeconds,
    MinutesOutOfRange,
    SecondsOutOfRange,
    LatitudeOutOfRange,
    LongitudeOutOfRange,
    ConflictingHemisphere,
}

impl core::fmt::Display for DmsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "empty input string"),
            Self::InvalidFormat => write!(f, "invalid DMS format"),
            Self::InvalidDegrees => write!(f, "invalid degrees component"),
            Self::InvalidMinutes => write!(f, "invalid minutes component"),
            Self::InvalidSeconds => write!(f, "invalid seconds component"),
            Self::MinutesOutOfRange => write!(f, "minutes out of range [0, 60)"),
            Self::SecondsOutOfRange => write!(f, "seconds out of range [0, 60)"),
            Self::LatitudeOutOfRange => write!(f, "latitude out of range [-90, 90]"),
            Self::LongitudeOutOfRange => write!(f, "longitude out of range [-180, 180]"),
            Self::ConflictingHemisphere => write!(f, "conflicting hemisphere or sign"),
        }
    }
}

impl std::error::Error for DmsError {}

/// Represents a coordinate component in degrees, minutes, and seconds with hemisphere.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dms {
    pub degrees: u32,
    pub minutes: u32,
    pub seconds: f64,
    pub hemisphere: char,
}

impl Dms {
    /// Create a new DMS component with validation.
    pub fn new(degrees: u32, minutes: u32, seconds: f64, hemisphere: char) -> Result<Self, DmsError> {
        let h = hemisphere.to_ascii_uppercase();
        if !matches!(h, 'N' | 'S' | 'E' | 'W') {
            return Err(DmsError::InvalidFormat);
        }
        if minutes >= 60 {
            return Err(DmsError::MinutesOutOfRange);
        }
        if !seconds.is_finite() || seconds < 0.0 || seconds >= 60.0 {
            return Err(DmsError::SecondsOutOfRange);
        }
        Ok(Self { degrees, minutes, seconds, hemisphere: h })
    }

    /// Convert DMS component to signed decimal degrees.
    pub fn to_degrees(&self) -> f64 {
        let mag = self.degrees as f64 + (self.minutes as f64) / 60.0 + self.seconds / 3600.0;
        if matches!(self.hemisphere, 'S' | 'W') { -mag } else { mag }
    }

    /// Create a Dms instance from decimal latitude in [-90, 90].
    pub fn from_degrees_lat(lat: f64) -> Result<Self, DmsError> {
        if !(-90.0..=90.0).contains(&lat) {
            return Err(DmsError::LatitudeOutOfRange);
        }
        let hemi = if lat >= 0.0 { 'N' } else { 'S' };
        Self::decompose(lat.abs(), hemi)
    }

    /// Create a Dms instance from decimal longitude in [-180, 180].
    pub fn from_degrees_lon(lon: f64) -> Result<Self, DmsError> {
        if !(-180.0..=180.0).contains(&lon) {
            return Err(DmsError::LongitudeOutOfRange);
        }
        let hemi = if lon >= 0.0 { 'E' } else { 'W' };
        Self::decompose(lon.abs(), hemi)
    }

    fn decompose(mag: f64, hemisphere: char) -> Result<Self, DmsError> {
        let mut deg = mag.floor() as u32;
        let rem_min = (mag - deg as f64) * 60.0;
        let mut min = rem_min.floor() as u32;
        let mut sec = (rem_min - min as f64) * 60.0;
        if sec >= 59.9999995 {
            sec = 0.0;
            min += 1;
            if min >= 60 {
                min = 0;
                deg += 1;
            }
        }
        Ok(Self { degrees: deg, minutes: min, seconds: sec, hemisphere })
    }
}

impl core::fmt::Display for Dms {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}° {:02}' {:05.2}\" {}",
            self.degrees, self.minutes, self.seconds, self.hemisphere
        )
    }
}

/// Format decimal latitude into standard DMS representation.
pub fn format_dms_lat(lat: f64) -> Result<String, DmsError> {
    Ok(Dms::from_degrees_lat(lat)?.to_string())
}

/// Format decimal longitude into standard DMS representation.
pub fn format_dms_lon(lon: f64) -> Result<String, DmsError> {
    Ok(Dms::from_degrees_lon(lon)?.to_string())
}

/// Format lat/lon into standard DMS representation.
pub fn format_dms(lat: f64, lon: f64) -> Result<String, DmsError> {
    Ok(format!("{}, {}", format_dms_lat(lat)?, format_dms_lon(lon)?))
}

fn tokenize(s: &str) -> Vec<String> {
    let mut normalized = String::with_capacity(s.len() + 4);
    let mut prev = ' ';
    for ch in s.chars() {
        if matches!(ch, '°' | '\'' | '"' | '′' | '″' | ',' | ';' | '\t' | '\n') {
            normalized.push(' ');
            prev = ' ';
        } else if ch.is_ascii_alphabetic() {
            if prev.is_ascii_digit() || prev == '.' {
                normalized.push(' ');
            }
            normalized.push(ch);
            prev = ch;
        } else if ch.is_ascii_digit() || ch == '+' || ch == '-' {
            if prev.is_ascii_alphabetic() {
                normalized.push(' ');
            }
            normalized.push(ch);
            prev = ch;
        } else {
            normalized.push(ch);
            prev = ch;
        }
    }
    normalized.split_whitespace().map(String::from).collect()
}

fn parse_hemi(tok: &str) -> Option<char> {
    if tok.len() == 1 {
        let c = tok.chars().next()?.to_ascii_uppercase();
        if matches!(c, 'N' | 'S' | 'E' | 'W') {
            return Some(c);
        }
    }
    None
}
