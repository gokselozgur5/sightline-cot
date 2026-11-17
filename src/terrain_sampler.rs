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
            rows,
            elevations,
            nodata_value: None,
        })
    }

    /// Construct grid from raw floats, treating NaN and nodata sentinel as None.
    pub fn from_raw(
        origin_x: f64,
        origin_y: f64,
        step_x: f64,
        step_y: f64,
        cols: usize,
        rows: usize,
        raw: &[f64],
        nodata: Option<f64>,
    ) -> Result<Self, TerrainSamplerError> {
        let is_nd = |v: f64| !v.is_finite() || nodata.is_some_and(|nd| (v - nd).abs() < 1e-5);
        let elevations: Vec<Option<f64>> = raw.iter().map(|&v| (!is_nd(v)).then_some(v)).collect();
        let mut grid = Self::new(origin_x, origin_y, step_x, step_y, cols, rows, elevations)?;
        grid.nodata_value = nodata;
        Ok(grid)
    }

    /// Construct a flat terrain grid with uniform elevation.
    pub fn flat(
        origin_x: f64,
        origin_y: f64,
        step_x: f64,
        step_y: f64,
        cols: usize,
        rows: usize,
        elevation: f64,
    ) -> Result<Self, TerrainSamplerError> {
        if !elevation.is_finite() {
            return Err(TerrainSamplerError::NonFiniteInput);
        }
        let elevations = vec![Some(elevation); cols * rows];
        Self::new(origin_x, origin_y, step_x, step_y, cols, rows, elevations)
    }

    /// Maximum X coordinate in grid coverage.
    #[inline]
    pub fn max_x(&self) -> f64 {
        self.origin_x + (self.cols - 1) as f64 * self.step_x
    }

    /// Maximum Y coordinate in grid coverage.
    #[inline]
    pub fn max_y(&self) -> f64 {
        self.origin_y + (self.rows - 1) as f64 * self.step_y
    }

    /// Check if continuous coordinate (x, y) falls within grid boundaries.
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.origin_x && x <= self.max_x() && y >= self.origin_y && y <= self.max_y()
    }

    /// Return bounding box (min_x, min_y, max_x, max_y).
    pub fn bounding_box(&self) -> (f64, f64, f64, f64) {
        (self.origin_x, self.origin_y, self.max_x(), self.max_y())
    }

    /// Return minimum and maximum valid elevations in the grid.
    pub fn min_max_elevation(&self) -> Option<(f64, f64)> {
        let mut min_h = f64::INFINITY;
        let mut max_h = f64::NEG_INFINITY;
        let mut found = false;
        for &opt in &self.elevations {
            if let Some(h) = opt {
                if h < min_h {
                    min_h = h;
                }
                if h > max_h {
                    max_h = h;
                }
                found = true;
            }
        }
        found.then_some((min_h, max_h))
    }

    /// Retrieve node elevation at discrete (col, row).
    pub fn get_node(&self, col: usize, row: usize) -> Option<f64> {
        if col < self.cols && row < self.rows {
            self.elevations[row * self.cols + col]
        } else {
            None
        }
    }

    /// Sample elevation using nearest neighbor node.
    pub fn sample_nearest(&self, x: f64, y: f64) -> Option<f64> {
        if !self.contains(x, y) {
            return None;
        }
        let u = ((x - self.origin_x) / self.step_x).round() as usize;
        let v = ((y - self.origin_y) / self.step_y).round() as usize;
        let col = u.min(self.cols - 1);
        let row = v.min(self.rows - 1);
        self.get_node(col, row)
    }

    /// Sample elevation using bilinear interpolation across 4 cell corners.
    pub fn sample_bilinear(&self, x: f64, y: f64) -> Option<f64> {
        if !self.contains(x, y) {
            return None;
        }
        let u = ((x - self.origin_x) / self.step_x).max(0.0);
        let v = ((y - self.origin_y) / self.step_y).max(0.0);
        let col0 = (u.floor() as usize).min(self.cols - 2);
        let row0 = (v.floor() as usize).min(self.rows - 2);
        let s = (u - col0 as f64).clamp(0.0, 1.0);
        let t = (v - row0 as f64).clamp(0.0, 1.0);

        let z00 = self.get_node(col0, row0)?;
        let z10 = self.get_node(col0 + 1, row0)?;
        let z01 = self.get_node(col0, row0 + 1)?;
        let z11 = self.get_node(col0 + 1, row0 + 1)?;

        Some((1.0 - s) * (1.0 - t) * z00 + s * (1.0 - t) * z10 + (1.0 - s) * t * z01 + s * t * z11)
    }

    /// Sample elevation using specified interpolation method.
    pub fn sample(&self, x: f64, y: f64, method: InterpolationMethod) -> Option<f64> {
        match method {
            InterpolationMethod::Bilinear => self.sample_bilinear(x, y),
            InterpolationMethod::NearestNeighbor => self.sample_nearest(x, y),
        }
    }

    /// Calculate terrain elevation gradient (dz/dx, dz/dy) at (x, y).
    pub fn surface_gradient(&self, x: f64, y: f64) -> Option<(f64, f64)> {
        let eps_x = 0.05 * self.step_x;
        let eps_y = 0.05 * self.step_y;
        let x_lo = (x - eps_x).max(self.origin_x);
        let x_hi = (x + eps_x).min(self.max_x());
        let y_lo = (y - eps_y).max(self.origin_y);
        let y_hi = (y + eps_y).min(self.max_y());

        let dx = x_hi - x_lo;
        let dy = y_hi - y_lo;
        if dx < 1e-9 || dy < 1e-9 {
            return None;
        }

        let z_x_lo = self.sample_bilinear(x_lo, y)?;
        let z_x_hi = self.sample_bilinear(x_hi, y)?;
        let z_y_lo = self.sample_bilinear(x, y_lo)?;
        let z_y_hi = self.sample_bilinear(x, y_hi)?;

        Some(((z_x_hi - z_x_lo) / dx, (z_y_hi - z_y_lo) / dy))
    }

    /// Compute upward unit surface normal vector [nx, ny, nz] at (x, y).
    pub fn surface_normal(&self, x: f64, y: f64) -> Option<[f64; 3]> {
        let (gx, gy) = self.surface_gradient(x, y)?;
        let norm = (gx * gx + gy * gy + 1.0).sqrt();
        Some([-gx / norm, -gy / norm, 1.0 / norm])
    }

    /// Compute terrain slope in degrees [0.0, 90.0] at (x, y).
    pub fn slope_deg(&self, x: f64, y: f64) -> Option<f64> {
        let (gx, gy) = self.surface_gradient(x, y)?;
        Some((gx * gx + gy * gy).sqrt().atan().to_degrees())
    }

    /// Compute terrain aspect compass azimuth in degrees [0.0, 360.0) at (x, y).
