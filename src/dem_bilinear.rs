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
