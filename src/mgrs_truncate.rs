//! MGRS grid reference precision truncation.

/// Error conditions in MGRS truncation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TruncateError {
    InvalidFormat,
    TargetPrecisionExceedsInput,
}

/// Target precision for MGRS truncation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MgrsPrecision {
    Point100k,
    Point10k,
    Point1k,
    Point100m,
    Point10m,
    Point1m,
}

impl MgrsPrecision {
    /// Number of digits per coordinate axis (0 to 5).
    pub fn digits_per_axis(&self) -> usize {
        match self {
            Self::Point100k => 0,
            Self::Point10k => 1,
            Self::Point1k => 2,
            Self::Point100m => 3,
            Self::Point10m => 4,
            Self::Point1m => 5,
        }
    }
}
