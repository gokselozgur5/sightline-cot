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
