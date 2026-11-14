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

impl LosRay {
    /// Create and validate a line-of-sight ray.
    pub fn new(origin: [f64; 3], direction: [f64; 3]) -> Result<Self, LosGroundError> {
        if !origin.iter().all(|x| x.is_finite()) || !direction.iter().all(|x| x.is_finite()) {
            return Err(LosGroundError::NonFiniteInput);
        }
        let mag_sq = direction[0] * direction[0]
            + direction[1] * direction[1]
            + direction[2] * direction[2];
        if mag_sq <= 1e-18 {
            return Err(LosGroundError::DegenerateRayDirection);
        }
        Ok(Self { origin, direction })
    }
}

/// Result of a successful line-of-sight ground intersection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LosGroundIntersection {
    /// 3D coordinates of intersection [east, north, up].
    pub point: [f64; 3],
    /// Slant range from origin to ground point in meters.
    pub slant_range_m: f64,
    /// Distance parameter t along the ray direction vector.
    pub t: f64,
}
