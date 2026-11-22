//! Digital elevation model bilinear interpolation with no-data handling.

/// Error conditions in DEM raster configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemError {
    InvalidDimensions,
    InvalidStep,
    NonFiniteCoordinate,
    DataLengthMismatch,
}

impl std::fmt::Display for DemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDimensions => write!(f, "grid dimensions must be at least 2x2"),
            Self::InvalidStep => write!(f, "cell step must be positive and finite"),
            Self::NonFiniteCoordinate => write!(f, "origin coordinates must be finite"),
            Self::DataLengthMismatch => write!(f, "elevations count must match cols * rows"),
        }
    }
}

impl std::error::Error for DemError {}

/// Regular 2D DEM raster supporting optional elevation values.
#[derive(Debug, Clone, PartialEq)]
pub struct DemRaster {
    pub origin_e_m: f64,
    pub origin_n_m: f64,
    pub cell_step_m: f64,
    pub cols: usize,
    pub rows: usize,
    pub elevations: Vec<Option<f64>>,
}

pub type DemBilinearSampler = DemRaster;
pub type DemBilinear = DemRaster;

impl DemRaster {
    /// Validate grid parameters prior to construction.
    fn validate(e0: f64, n0: f64, step: f64, c: usize, r: usize, n: usize) -> Result<(), DemError> {
        if !e0.is_finite() || !n0.is_finite() {
            return Err(DemError::NonFiniteCoordinate);
        }
        if !step.is_finite() || step <= 0.0 {
            return Err(DemError::InvalidStep);
        }
        if c < 2 || r < 2 {
            return Err(DemError::InvalidDimensions);
        }
        let expected_len = c.checked_mul(r).ok_or(DemError::DataLengthMismatch)?;
        if n != expected_len {
            return Err(DemError::DataLengthMismatch);
        }
        Ok(())
    }
}

impl DemRaster {
    /// Construct a validated DemRaster grid.
    pub fn new(
        e0: f64,
        n0: f64,
        step: f64,
        cols: usize,
        rows: usize,
        elevations: Vec<Option<f64>>,
    ) -> Result<Self, DemError> {
        Self::validate(e0, n0, step, cols, rows, elevations.len())?;
        let (origin_e_m, origin_n_m, cell_step_m) = (e0, n0, step);
        Ok(Self {
            origin_e_m,
            origin_n_m,
            cell_step_m,
            cols,
            rows,
            elevations,
        })
    }
}

impl DemRaster {
    /// Create DemRaster from raw floats mapping NaN and sentinel to None.
    pub fn from_f64(
        e0: f64,
        n0: f64,
        step: f64,
        cols: usize,
        rows: usize,
        raw: &[f64],
        nodata: Option<f64>,
    ) -> Result<Self, DemError> {
        let is_nd = |h: f64| !h.is_finite() || nodata.is_some_and(|nd| (h - nd).abs() < 1e-6);
        let elevations = raw.iter().map(|&h| (!is_nd(h)).then_some(h)).collect();
        Self::new(e0, n0, step, cols, rows, elevations)
    }
}

impl DemRaster {
    /// Check if coordinates fall within the grid coverage.
    pub fn contains_en(&self, e_m: f64, n_m: f64) -> bool {
        let max_e = self.origin_e_m + (self.cols - 1) as f64 * self.cell_step_m;
        let max_n = self.origin_n_m + (self.rows - 1) as f64 * self.cell_step_m;
        e_m >= self.origin_e_m && e_m <= max_e && n_m >= self.origin_n_m && n_m <= max_n
    }

    /// Retrieve elevation at discrete grid coordinates (col, row).
    pub fn get_node(&self, col: usize, row: usize) -> Option<f64> {
        (col < self.cols && row < self.rows).then(|| self.elevations[row * self.cols + col])?
    }
}

/// Bilinear interpolation across four cell nodes.
#[inline]
pub fn bilinear_interpolate_4(z00: f64, z10: f64, z01: f64, z11: f64, s: f64, t: f64) -> f64 {
    (1.0 - s) * (1.0 - t) * z00 + s * (1.0 - t) * z10 + (1.0 - s) * t * z01 + s * t * z11
}

/// Bilinear grid sampler over continuous cell coordinates (u, v).
pub fn sample_bilinear_cells<F: Fn(usize, usize) -> Option<f64>>(
    u: f64,
    v: f64,
    cols: usize,
    rows: usize,
    wrap_x: bool,
    read: F,
) -> Option<f64> {
    if cols < 2 || rows < 2 {
        return None;
    }
    let (u_pos, v_pos) = (u.max(0.0), v.max(0.0));
    let (c0, c1, s) = if wrap_x {
        let (c0, s) = (
            (u_pos.floor() as usize) % cols,
            (u_pos - u_pos.floor()).clamp(0.0, 1.0),
        );
        (c0, (c0 + 1) % cols, s)
    } else {
        let c0 = (u_pos.floor() as usize).min(cols - 2);
        (c0, c0 + 1, (u_pos - c0 as f64).clamp(0.0, 1.0))
    };
    let (r0, r1, t) = {
        let r0 = (v_pos.floor() as usize).min(rows - 2);
        (r0, r1_clamp(r0, rows), (v_pos - r0 as f64).clamp(0.0, 1.0))
    };
    Some(bilinear_interpolate_4(
        read(c0, r0)?,
        read(c1, r0)?,
        read(c0, r1)?,
        read(c1, r1)?,
        s,
        t,
    ))
}

#[inline]
fn r1_clamp(r0: usize, rows: usize) -> usize {
    (r0 + 1).min(rows - 1)
}

impl DemRaster {
    /// Bilinear elevation sample at continuous coordinates (e_m, n_m).
    pub fn sample(&self, e_m: f64, n_m: f64) -> Option<f64> {
        if !self.contains_en(e_m, n_m) {
            return None;
        }
        let u = ((e_m - self.origin_e_m) / self.cell_step_m).max(0.0);
        let v = ((n_m - self.origin_n_m) / self.cell_step_m).max(0.0);
        let (c0, r0) = (
            (u.floor() as usize).min(self.cols - 2),
            (v.floor() as usize).min(self.rows - 2),
        );
        let (s, t) = (
            (u - c0 as f64).clamp(0.0, 1.0),
            (v - r0 as f64).clamp(0.0, 1.0),
        );
        let (z00, z10) = (self.get_node(c0, r0)?, self.get_node(c0 + 1, r0)?);
        let (z01, z11) = (self.get_node(c0, r0 + 1)?, self.get_node(c0 + 1, r0 + 1)?);
        Some((1.0 - s) * (1.0 - t) * z00 + s * (1.0 - t) * z10 + (1.0 - s) * t * z01 + s * t * z11)
    }

    /// Alias for sample to match DemGrid interface.
    pub fn elevation_at(&self, e_m: f64, n_m: f64) -> Option<f64> {
        self.sample(e_m, n_m)
    }
}

impl From<&crate::camera::DemGrid> for DemRaster {
    fn from(grid: &crate::camera::DemGrid) -> Self {
        Self {
            origin_e_m: grid.origin_e_m,
            origin_n_m: grid.origin_n_m,
            cell_step_m: grid.cell_step_m,
            cols: grid.cols,
            rows: grid.rows,
            elevations: grid.elevations.iter().map(|&h| Some(h)).collect(),
        }
    }
}

/// Standalone helper for bilinear sampling.
pub fn sample_dem_bilinear(dem: &DemRaster, e_m: f64, n_m: f64) -> Option<f64> {
    dem.sample(e_m, n_m)
}
