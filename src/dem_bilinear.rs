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
        if n != c * r {
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
