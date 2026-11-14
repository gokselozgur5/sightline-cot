//! Line-of-sight ray ground and planar surface intersection.

/// Errors occurring during line-of-sight ground intersection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LosGroundError {
    /// Ray direction is horizontal, pointing upwards, or away from ground.
    RayAboveOrParallelGround,
    /// Ray direction vector has zero or near-zero magnitude.
    DegenerateRayDirection,
    /// One or more coordinates or vector components are non-finite.
    NonFiniteInput,
}

impl core::fmt::Display for LosGroundError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RayAboveOrParallelGround => {
                write!(f, "ray is pointing above or parallel to ground")
            }
            Self::DegenerateRayDirection => write!(f, "ray direction vector is degenerate"),
            Self::NonFiniteInput => write!(f, "input coordinates are non-finite"),
        }
    }
}

impl std::error::Error for LosGroundError {}

/// 3D ray defined by origin point and direction vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LosRay {
    pub origin: [f64; 3],
    pub direction: [f64; 3],
}
