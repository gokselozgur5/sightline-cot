//! Digital elevation model bilinear interpolation with no-data handling.

/// Error conditions in DEM raster configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemError {
    InvalidDimensions,
    InvalidStep,
    NonFiniteCoordinate,
    DataLengthMismatch,
}
