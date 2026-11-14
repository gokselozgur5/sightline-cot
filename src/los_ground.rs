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
