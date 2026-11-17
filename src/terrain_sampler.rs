//! GeoTIFF elevation grid sampler and terrain line-of-sight intercept solver.

use core::fmt;

/// Errors arising during terrain sampling and ray intersection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainSamplerError {
    /// Non-finite coordinate, elevation, or ray vector component.
    NonFiniteInput,
    /// Grid dimensions must be at least 2x2.
    InvalidDimensions,
    /// Grid pixel resolution/step must be positive and finite.
    InvalidResolution,
    /// Data length does not match cols * rows.
    DataLengthMismatch,
    /// Point is outside the defined grid bounds.
    OutOfBounds,
    /// Ray direction has zero magnitude.
    DegenerateRayDirection,
    /// Ray points away from terrain or parallel above maximum elevation.
    RayPointsAway,
    /// Ray does not intersect the elevation surface within max range.
    NoIntersection,
    /// Intercept solver reached max iterations without meeting convergence tolerance.
