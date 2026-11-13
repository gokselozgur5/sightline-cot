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
