//! 3D Constant-Velocity Kalman Tracker.

/// Error types occurring during multi-target tracking operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackerError {
    /// Non-finite numerical state or detection coordinates encountered.
    NonFiniteInput,
    /// Invalid time delta (negative or non-finite).
    InvalidDeltaTime,
}

impl std::fmt::Display for TrackerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteInput => write!(f, "non-finite input encountered"),
            Self::InvalidDeltaTime => write!(f, "invalid delta time"),
        }
    }
}

impl std::error::Error for TrackerError {}
