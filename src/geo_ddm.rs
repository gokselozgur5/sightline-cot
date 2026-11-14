//! Degrees decimal-minutes (DDM) coordinate parser.

/// Errors that can occur during DDM coordinate parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DdmError {
    EmptyInput,
    InvalidFormat,
    InvalidDegrees,
    InvalidMinutes,
    MinutesOutOfRange,
    LatitudeOutOfRange,
    LongitudeOutOfRange,
    ConflictingHemisphere,
}

impl core::fmt::Display for DdmError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "empty input string"),
            Self::InvalidFormat => write!(f, "invalid DDM format"),
            Self::InvalidDegrees => write!(f, "invalid degrees component"),
            Self::InvalidMinutes => write!(f, "invalid minutes component"),
            Self::MinutesOutOfRange => write!(f, "minutes out of range [0, 60)"),
            Self::LatitudeOutOfRange => write!(f, "latitude out of range [-90, 90]"),
            Self::LongitudeOutOfRange => write!(f, "longitude out of range [-180, 180]"),
            Self::ConflictingHemisphere => write!(f, "conflicting hemisphere or sign"),
        }
    }
}

impl std::error::Error for DdmError {}

fn tokenize(s: &str) -> Vec<String> {
    let mut normalized = String::with_capacity(s.len() + 4);
    let mut prev = ' ';
    for ch in s.chars() {
        if matches!(ch, '°' | '\'' | '"' | ',' | ';' | '\t' | '\n') {
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
        let ch = tok.chars().next()?.to_ascii_uppercase();
        if matches!(ch, 'N' | 'S' | 'E' | 'W') {
            return Some(ch);
        }
    }
    None
}

fn split_components(toks: &[String]) -> Result<(&[String], &[String]), DdmError> {
    match toks.len() {
        4 => Ok((&toks[0..2], &toks[2..4])),
        6 => Ok((&toks[0..3], &toks[3..6])),
        5 => {
            if parse_hemi(&toks[0]).is_some() {
                Ok((&toks[0..3], &toks[3..5]))
            } else if parse_hemi(&toks[4]).is_some()
                || matches!(parse_hemi(&toks[2]), Some('E' | 'W'))
            {
                Ok((&toks[0..2], &toks[2..5]))
            } else if parse_hemi(&toks[2]).is_some() {
                Ok((&toks[0..3], &toks[3..5]))
            } else {
                Err(DdmError::InvalidFormat)
            }
        }
        _ => Err(DdmError::InvalidFormat),
    }
}

fn parse_comp(toks: &[String]) -> Result<(f64, Option<char>), DdmError> {
    let (h, deg_s, min_s) = match toks {
        [t0, t1] => (None, t0.as_str(), t1.as_str()),
        [t0, t1, t2] if parse_hemi(t0).is_some() => (parse_hemi(t0), t1.as_str(), t2.as_str()),
        [t0, t1, t2] if parse_hemi(t2).is_some() => (parse_hemi(t2), t0.as_str(), t1.as_str()),
        _ => return Err(DdmError::InvalidFormat),
    };
    let deg: f64 = deg_s.parse().map_err(|_| DdmError::InvalidDegrees)?;
    let min: f64 = min_s.parse().map_err(|_| DdmError::InvalidMinutes)?;
    if !deg.is_finite() {
        return Err(DdmError::InvalidDegrees);
    }
    if !min.is_finite() || min < 0.0 || min >= 60.0 {
        return Err(DdmError::MinutesOutOfRange);
    }
    let is_neg = deg_s.starts_with('-');
    if h.is_some() && is_neg {
        return Err(DdmError::ConflictingHemisphere);
    }
    let mag = deg.abs() + min / 60.0;
    let signed = match h {
        Some('S' | 'W') => -mag,
        Some(_) => mag,
        None if is_neg => -mag,
        None => mag,
    };
    Ok((signed, h))
}

/// Parses a degrees decimal-minutes coordinate string into (lat_deg, lon_deg).
pub fn parse_ddm(s: &str) -> Result<(f64, f64), DdmError> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(DdmError::EmptyInput);
    }
    let toks = tokenize(trimmed);
    let (toks1, toks2) = split_components(&toks)?;
    let (v1, h1) = parse_comp(toks1)?;
    let (v2, h2) = parse_comp(toks2)?;
    let (lat, lon) = match (h1, h2) {
        (Some('N' | 'S'), Some('E' | 'W')) => (v1, v2),
        (Some('E' | 'W'), Some('N' | 'S')) => (v2, v1),
        (Some('N' | 'S'), None) => (v1, v2),
        (Some('E' | 'W'), None) => (v2, v1),
        (None, Some('E' | 'W')) => (v1, v2),
        (None, Some('N' | 'S')) => (v2, v1),
        (None, None) => (v1, v2),
        _ => return Err(DdmError::ConflictingHemisphere),
    };
    if !(-90.0..=90.0).contains(&lat) {
        return Err(DdmError::LatitudeOutOfRange);
    }
    if !(-180.0..=180.0).contains(&lon) {
        return Err(DdmError::LongitudeOutOfRange);
    }
    Ok((lat, lon))
}
