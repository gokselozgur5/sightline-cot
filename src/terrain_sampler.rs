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
    MaxIterationsReached,
}

impl fmt::Display for TerrainSamplerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput => write!(f, "non-finite input value encountered"),
            Self::InvalidDimensions => write!(f, "grid dimensions must be at least 2x2"),
            Self::InvalidResolution => {
                write!(f, "grid resolution step must be positive and finite")
            }
            Self::DataLengthMismatch => {
                write!(f, "elevation data length must equal cols * rows")
            }
            Self::OutOfBounds => write!(f, "coordinate falls outside elevation grid bounds"),
            Self::DegenerateRayDirection => {
                write!(f, "ray direction vector magnitude is zero")
            }
            Self::RayPointsAway => {
                write!(
                    f,
                    "ray points away from terrain or is parallel above surface"
                )
            }
            Self::NoIntersection => {
                write!(f, "ray does not intersect terrain surface within range")
            }
            Self::MaxIterationsReached => {
                write!(f, "terrain intercept solver exceeded maximum iterations")
            }
        }
    }
}

impl std::error::Error for TerrainSamplerError {}

/// Elevation interpolation method across grid cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InterpolationMethod {
    /// Standard bilinear interpolation across 4 surrounding nodes.
    #[default]
    Bilinear,
    /// Nearest neighbor node sampling.
    NearestNeighbor,
}

/// Regular 2D GeoTIFF-style elevation grid.
#[derive(Debug, Clone, PartialEq)]
pub struct GeoTiffElevationGrid {
    /// Minimum X coordinate (e.g. westernmost longitude or easting).
    pub origin_x: f64,
    /// Minimum Y coordinate (e.g. southernmost latitude or northing).
    pub origin_y: f64,
    /// Cell step size along X dimension (positive).
    pub step_x: f64,
    /// Cell step size along Y dimension (positive).
    pub step_y: f64,
    /// Number of columns (cells along X).
    pub cols: usize,
    /// Number of rows (cells along Y).
    pub rows: usize,
    /// Elevation values in row-major order (row * cols + col).
    pub elevations: Vec<Option<f64>>,
    /// Sentinel nodata value.
    pub nodata_value: Option<f64>,
}

impl GeoTiffElevationGrid {
    /// Construct a validated GeoTIFF elevation grid.
    pub fn new(
        origin_x: f64,
        origin_y: f64,
        step_x: f64,
        step_y: f64,
        cols: usize,
        rows: usize,
        elevations: Vec<Option<f64>>,
    ) -> Result<Self, TerrainSamplerError> {
        if !origin_x.is_finite() || !origin_y.is_finite() {
            return Err(TerrainSamplerError::NonFiniteInput);
        }
        if !step_x.is_finite() || step_x <= 0.0 || !step_y.is_finite() || step_y <= 0.0 {
            return Err(TerrainSamplerError::InvalidResolution);
        }
        if cols < 2 || rows < 2 {
            return Err(TerrainSamplerError::InvalidDimensions);
        }
        if elevations.len() != cols * rows {
            return Err(TerrainSamplerError::DataLengthMismatch);
        }
        Ok(Self {
            origin_x,
            origin_y,
            step_x,
            step_y,
            cols,
