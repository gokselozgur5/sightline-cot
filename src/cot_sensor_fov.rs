//! CoT sensor field-of-view (FOV) frustum and cone calculator.

/// Errors occurring during sensor FOV calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorFovError {
    NonFiniteParameter,
    InvalidFovAngle,
    InvalidRange,
    InvalidElevation,
    RayDoesNotIntersectGround,
}
