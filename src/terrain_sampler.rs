//! Elevation grid sampler and terrain line-of-sight intercept solver.
//!
//! Note: Despite the historical naming of [`GeoTiffElevationGrid`], this module
//! does not provide GeoTIFF file decoding. Grids are in-memory buffers constructed
//! from caller-supplied slices or converted from DEM structures.

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

/// Regular 2D in-memory elevation grid.
///
/// Despite the name, this struct does not perform GeoTIFF decoding or file I/O;
/// it holds an in-memory elevation raster buffer.
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
    #[allow(clippy::too_many_arguments)]
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
        let u = (x - self.origin_x) / self.step_x;
        let v = (y - self.origin_y) / self.step_y;
        crate::dem_bilinear::sample_bilinear_cells(u, v, self.cols, self.rows, false, |c, r| {
            self.get_node(c, r)
        })
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
    pub fn aspect_deg(&self, x: f64, y: f64) -> Option<f64> {
        let (gx, gy) = self.surface_gradient(x, y)?;
        if gx.abs() < 1e-9 && gy.abs() < 1e-9 {
            return Some(0.0);
        }
        let rad = (-gy).atan2(-gx);
        let mut deg = 90.0 - rad.to_degrees();
        if deg < 0.0 {
            deg += 360.0;
        }
        if deg >= 360.0 {
            deg -= 360.0;
        }
        Some(deg)
    }
}

/// 3D ray in the elevation coordinate frame [X, Y, Z].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TerrainRay {
    /// Ray origin point [X, Y, Z].
    pub origin: [f64; 3],
    /// Normalized direction unit vector.
    pub direction: [f64; 3],
}

impl TerrainRay {
    /// Create and normalize a new line-of-sight ray.
    pub fn new(origin: [f64; 3], direction: [f64; 3]) -> Result<Self, TerrainSamplerError> {
        if !origin.iter().all(|v| v.is_finite()) || !direction.iter().all(|v| v.is_finite()) {
            return Err(TerrainSamplerError::NonFiniteInput);
        }
        let max_abs = direction[0]
            .abs()
            .max(direction[1].abs())
            .max(direction[2].abs());
        if max_abs == 0.0 {
            return Err(TerrainSamplerError::DegenerateRayDirection);
        }
        let scaled = [
            direction[0] / max_abs,
            direction[1] / max_abs,
            direction[2] / max_abs,
        ];
        let scaled_mag =
            (scaled[0] * scaled[0] + scaled[1] * scaled[1] + scaled[2] * scaled[2]).sqrt();
        let direction = [
            scaled[0] / scaled_mag,
            scaled[1] / scaled_mag,
            scaled[2] / scaled_mag,
        ];
        Ok(Self { origin, direction })
    }

    /// Calculate 3D coordinate along the ray at propagation parameter t.
    #[inline]
    pub fn point_at(&self, t: f64) -> [f64; 3] {
        [
            self.origin[0] + t * self.direction[0],
            self.origin[1] + t * self.direction[1],
            self.origin[2] + t * self.direction[2],
        ]
    }
}

/// Result of a line-of-sight terrain intersection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TerrainIntercept {
    /// 3D coordinates [X, Y, Z] of intercept.
    pub point: [f64; 3],
    /// Slant range distance from ray origin to intercept.
    pub distance: f64,
    /// Upward unit normal vector at surface intercept.
    pub surface_normal: [f64; 3],
    /// Terrain slope at intercept in degrees [0.0, 90.0].
    pub slope_deg: f64,
    /// Incident angle between ray direction and surface normal in degrees.
    pub incident_angle_deg: f64,
    /// Number of bisection iterations performed.
    pub iterations: usize,
}

/// Line-of-sight terrain intercept solver.
#[derive(Debug, Clone, PartialEq)]
pub struct TerrainInterceptSolver {
    /// Ray marching step size.
    pub step_size: f64,
    /// Maximum search distance along ray.
    pub max_range: f64,
    /// Height convergence tolerance.
    pub tolerance: f64,
    /// Maximum bisection refinement iterations.
    pub max_iterations: usize,
}

impl Default for TerrainInterceptSolver {
    fn default() -> Self {
        Self {
            step_size: 10.0,
            max_range: 50_000.0,
            tolerance: 0.05,
            max_iterations: 64,
        }
    }
}

impl TerrainInterceptSolver {
    /// Create a configured terrain intercept solver.
    pub fn new(
        step_size: f64,
        max_range: f64,
        tolerance: f64,
        max_iterations: usize,
    ) -> Result<Self, TerrainSamplerError> {
        if !step_size.is_finite()
            || step_size <= 0.0
            || !max_range.is_finite()
            || max_range <= 0.0
            || !tolerance.is_finite()
            || tolerance <= 0.0
        {
            return Err(TerrainSamplerError::NonFiniteInput);
        }
        if max_iterations == 0 {
            return Err(TerrainSamplerError::InvalidDimensions);
        }
        Ok(Self {
            step_size,
            max_range,
            tolerance,
            max_iterations,
        })
    }

    /// Intersect a line-of-sight ray with the elevation grid.
    pub fn solve(
        &self,
        grid: &GeoTiffElevationGrid,
        ray: &TerrainRay,
    ) -> Result<TerrainIntercept, TerrainSamplerError> {
        let (min_elev, max_elev) = grid
            .min_max_elevation()
            .ok_or(TerrainSamplerError::NoIntersection)?;

        if ray.origin[2] > max_elev && ray.direction[2] >= 0.0 {
            return Err(TerrainSamplerError::RayPointsAway);
        }

        if let Some(elev) = grid.sample_bilinear(ray.origin[0], ray.origin[1]) {
            if ray.origin[2] < elev {
                return Err(TerrainSamplerError::NoIntersection);
            }
        }

        let mut t_prev = 0.0;
        let mut t_curr = 0.0;
        let step = self.step_size.max(0.1);
        let mut march_steps: usize = 0;

        while t_curr < self.max_range && march_steps < 100_000 {
            march_steps += 1;
            let next_t = (t_curr + step).min(self.max_range);
            if next_t <= t_curr {
                break;
            }
            t_curr = next_t;
            let p = ray.point_at(t_curr);

            if grid.contains(p[0], p[1]) {
                if let Some(elev) = grid.sample_bilinear(p[0], p[1]) {
                    if p[2] <= elev {
                        let mut low = t_prev;
                        let mut high = t_curr;
                        let mut best_t = high;
                        let mut converged = false;
                        let mut iters = 0;

                        while iters < self.max_iterations {
                            iters += 1;
                            let mid = 0.5 * (low + high);
                            let p_mid = ray.point_at(mid);

                            if let Some(mid_elev) = grid.sample_bilinear(p_mid[0], p_mid[1]) {
                                let diff = p_mid[2] - mid_elev;
                                if diff.abs() <= self.tolerance {
                                    best_t = mid;
                                    converged = true;
                                    break;
                                }
                                if diff > 0.0 {
                                    low = mid;
                                } else {
                                    high = mid;
                                    best_t = mid;
                                }
                            } else {
                                low = mid;
                            }

                            if (high - low) < 1e-4 {
                                best_t = 0.5 * (low + high);
                                converged = true;
                                break;
                            }
                        }

                        if !converged {
                            return Err(TerrainSamplerError::MaxIterationsReached);
                        }

                        let hit_point = ray.point_at(best_t);
                        let normal = grid
                            .surface_normal(hit_point[0], hit_point[1])
                            .unwrap_or([0.0, 0.0, 1.0]);
                        let slope = grid.slope_deg(hit_point[0], hit_point[1]).unwrap_or(0.0);
                        let dot = ray.direction[0] * normal[0]
                            + ray.direction[1] * normal[1]
                            + ray.direction[2] * normal[2];
                        let inc_deg = (-dot).clamp(-1.0, 1.0).acos().to_degrees();

                        return Ok(TerrainIntercept {
                            point: hit_point,
                            distance: best_t,
                            surface_normal: normal,
                            slope_deg: slope,
                            incident_angle_deg: inc_deg,
                            iterations: iters,
                        });
                    }
                }
            } else if p[2] < min_elev && ray.direction[2] <= 0.0 {
                break;
            }

            t_prev = t_curr;
            if t_curr >= self.max_range {
                break;
            }
        }

        Err(TerrainSamplerError::NoIntersection)
    }
}

/// Convert terrain intercept result to standard Cursor-on-Target event.
pub fn cot_target_from_intercept(
    intercept: &TerrainIntercept,
    uid: &str,
    cot_type: &str,
    time_ms: i64,
    stale_duration_ms: i64,
) -> crate::cot::CotEvent {
    crate::cot::CotEvent {
        uid: uid.to_string(),
        cot_type: cot_type.to_string(),
        how: "m-g".to_string(),
        time_ms,
        start_ms: time_ms,
        stale_ms: time_ms + stale_duration_ms,
        lat_deg: intercept.point[1],
        lon_deg: intercept.point[0],
        hae_m: intercept.point[2],
        ce_m: 5.0,
        le_m: 3.0,
        course_deg: 0.0,
        speed_mps: 0.0,
    }
}

impl From<&crate::dem_bilinear::DemRaster> for GeoTiffElevationGrid {
    fn from(dem: &crate::dem_bilinear::DemRaster) -> Self {
        Self {
            origin_x: dem.origin_e_m,
            origin_y: dem.origin_n_m,
            step_x: dem.cell_step_m,
            step_y: dem.cell_step_m,
            cols: dem.cols,
            rows: dem.rows,
            elevations: dem.elevations.clone(),
            nodata_value: None,
        }
    }
}

impl From<&crate::camera::DemGrid> for GeoTiffElevationGrid {
    fn from(grid: &crate::camera::DemGrid) -> Self {
        Self {
            origin_x: grid.origin_e_m,
            origin_y: grid.origin_n_m,
            step_x: grid.cell_step_m,
            step_y: grid.cell_step_m,
            cols: grid.cols,
            rows: grid.rows,
            elevations: grid.elevations.iter().map(|&h| Some(h)).collect(),
            nodata_value: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation_and_bounds() {
        let elev = vec![Some(100.0), Some(120.0), Some(140.0), Some(160.0)];
        let grid = GeoTiffElevationGrid::new(10.0, 20.0, 1.0, 2.0, 2, 2, elev).unwrap();
        assert_eq!(grid.bounding_box(), (10.0, 20.0, 11.0, 22.0));
        assert!(grid.contains(10.5, 21.0));
        assert!(!grid.contains(9.9, 21.0));
        assert_eq!(grid.min_max_elevation(), Some((100.0, 160.0)));
    }

    #[test]
    fn test_bilinear_and_nearest_sampling() {
        let elev = vec![Some(10.0), Some(20.0), Some(30.0), Some(40.0)];
        let grid = GeoTiffElevationGrid::new(0.0, 0.0, 10.0, 10.0, 2, 2, elev).unwrap();

        let mid = grid.sample_bilinear(5.0, 5.0).unwrap();
        assert!((mid - 25.0).abs() < 1e-6);

        let near_orig = grid.sample_nearest(2.0, 2.0).unwrap();
        assert_eq!(near_orig, 10.0);

        let near_corner = grid.sample_nearest(8.0, 8.0).unwrap();
        assert_eq!(near_corner, 40.0);
    }

    #[test]
    fn test_gradient_slope_normal() {
        let elev = vec![Some(0.0), Some(10.0), Some(0.0), Some(10.0)];
        let grid = GeoTiffElevationGrid::new(0.0, 0.0, 10.0, 10.0, 2, 2, elev).unwrap();

        let (gx, gy) = grid.surface_gradient(5.0, 5.0).unwrap();
        assert!((gx - 1.0).abs() < 1e-3);
        assert!(gy.abs() < 1e-3);

        let slope = grid.slope_deg(5.0, 5.0).unwrap();
        assert!((slope - 45.0).abs() < 1e-1);

        let normal = grid.surface_normal(5.0, 5.0).unwrap();
        assert!(normal[2] > 0.0);
        assert!(
            (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2] - 1.0).abs()
                < 1e-6
        );
    }

    #[test]
    fn test_terrain_ray_intercept_flat() {
        let grid = GeoTiffElevationGrid::flat(0.0, 0.0, 10.0, 10.0, 11, 11, 50.0).unwrap();
        let ray = TerrainRay::new([50.0, 50.0, 150.0], [0.0, 0.0, -1.0]).unwrap();
        let solver = TerrainInterceptSolver::default();
        let intercept = solver.solve(&grid, &ray).unwrap();

        assert!((intercept.point[0] - 50.0).abs() < 0.1);
        assert!((intercept.point[1] - 50.0).abs() < 0.1);
        assert!((intercept.point[2] - 50.0).abs() < 0.1);
        assert!((intercept.distance - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_cot_event_conversion() {
        let intercept = TerrainIntercept {
            point: [32.5, 39.9, 850.0],
            distance: 1250.0,
            surface_normal: [0.0, 0.0, 1.0],
            slope_deg: 0.0,
            incident_angle_deg: 30.0,
            iterations: 8,
        };
        let cot = cot_target_from_intercept(&intercept, "TGT-01", "a-h-G", 1000, 60000);
        assert_eq!(cot.uid, "TGT-01");
        assert_eq!(cot.cot_type, "a-h-G");
        assert_eq!(cot.lat_deg, 39.9);
        assert_eq!(cot.lon_deg, 32.5);
        assert_eq!(cot.hae_m, 850.0);
    }
}
