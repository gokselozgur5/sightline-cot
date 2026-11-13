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

/// Truncate an MGRS coordinate string to a coarser precision.
pub fn truncate_mgrs(mgrs: &str, target: MgrsPrecision) -> Result<String, TruncateError> {
    let s = mgrs.trim();
    let num_digits = s.chars().take_while(|c| c.is_ascii_digit()).count();
    if !(1..=2).contains(&num_digits) || s.len() < num_digits + 3 {
        return Err(TruncateError::InvalidFormat);
    }
    let (prefix, numerical) = s.split_at(num_digits + 3);
    if numerical.len() % 2 != 0 || !numerical.chars().all(|c| c.is_ascii_digit()) {
        return Err(TruncateError::InvalidFormat);
    }
    let in_digits = numerical.len() / 2;
    let target_digits = target.digits_per_axis();
    if target_digits > in_digits {
        return Err(TruncateError::TargetPrecisionExceedsInput);
    }
    let (e, n) = numerical.split_at(in_digits);
    Ok(format!("{}{}{}", prefix, &e[..target_digits], &n[..target_digits]))
}
