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
