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
