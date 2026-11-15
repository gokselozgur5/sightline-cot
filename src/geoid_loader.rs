//! ASCII grid format loader for geoid undulation tables.

/// Error conditions during ASCII geoid grid parsing and sampling.
#[derive(Debug, Clone, PartialEq)]
pub enum GeoidError {
    EmptyInput,
    InvalidHeader(String),
    MissingHeader(&'static str),
    InvalidDimensions { cols: usize, rows: usize },
    InvalidCellSize(f64),
    InvalidDataValue(String),
    DataCountMismatch { expected: usize, found: usize },
}

impl std::fmt::Display for GeoidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "empty ascii grid input"),
            Self::InvalidHeader(m) => write!(f, "invalid ascii header: {}", m),
            Self::MissingHeader(h) => write!(f, "missing required header: {}", h),
            Self::InvalidDimensions { cols, rows } => {
                write!(f, "dimensions must be >= 1, got {}x{}", cols, rows)
            }
            Self::InvalidCellSize(s) => write!(f, "invalid cell size: {}", s),
            Self::InvalidDataValue(v) => write!(f, "invalid float token: {}", v),
            Self::DataCountMismatch { expected, found } => {
                write!(f, "expected {} values, found {}", expected, found)
            }
        }
    }
}

impl std::error::Error for GeoidError {}

/// Regular geoid undulation grid loaded from an ASCII raster.
#[derive(Debug, Clone, PartialEq)]
pub struct GeoidGrid {
    pub cols: usize,
    pub rows: usize,
    pub xllcorner: f64,
    pub yllcorner: f64,
    pub cellsize: f64,
    pub nodata_value: Option<f64>,
    pub data: Vec<Option<f64>>,
}

impl GeoidGrid {
    /// Construct and validate a new GeoidGrid raster instance.
    pub fn new(
        cols: usize, rows: usize, x0: f64, y0: f64, step: f64,
        nodata: Option<f64>, data: Vec<Option<f64>>,
    ) -> Result<Self, GeoidError> {
        if cols == 0 || rows == 0 {
            return Err(GeoidError::InvalidDimensions { cols, rows });
        }
        if !step.is_finite() || step <= 0.0 {
            return Err(GeoidError::InvalidCellSize(step));
        }
        if !x0.is_finite() || !y0.is_finite() {
            return Err(GeoidError::InvalidHeader("non-finite origin".into()));
        }
        if data.len() != cols * rows {
            let (expected, found) = (cols * rows, data.len());
            return Err(GeoidError::DataCountMismatch { expected, found });
        }
        Ok(Self { cols, rows, xllcorner: x0, yllcorner: y0, cellsize: step, nodata_value: nodata, data })
    }

    /// Retrieve undulation value at integer grid coordinate (col, row).
    pub fn get(&self, col: usize, row: usize) -> Option<f64> {
        (col < self.cols && row < self.rows).then(|| self.data[row * self.cols + col])?
    }

    /// Geographic coverage bounds in degrees.
    pub fn min_lon(&self) -> f64 { self.xllcorner }
    pub fn max_lon(&self) -> f64 { self.xllcorner + self.cols as f64 * self.cellsize }
    pub fn min_lat(&self) -> f64 { self.yllcorner }
    pub fn max_lat(&self) -> f64 { self.yllcorner + self.rows as f64 * self.cellsize }

    /// Test if longitude and latitude fall within the grid coverage.
    pub fn contains(&self, lon: f64, lat: f64) -> bool {
        lon >= self.min_lon() && lon <= self.max_lon() && lat >= self.min_lat() && lat <= self.max_lat()
    }

    /// Bilinear interpolation of geoid undulation at continuous geographic coordinates.
    pub fn undulation_at(&self, lon: f64, lat: f64) -> Option<f64> {
        if !self.contains(lon, lat) || !lon.is_finite() || !lat.is_finite() {
            return None;
        }
        if self.cols == 1 && self.rows == 1 { return self.get(0, 0); }
        let u = ((lon - self.xllcorner) / self.cellsize).max(0.0);
        let v_norm = ((lat - self.yllcorner) / self.cellsize).max(0.0);
        let v_top = (self.rows.saturating_sub(1) as f64 - v_norm).max(0.0);
        let c0 = (u.floor() as usize).min(self.cols.saturating_sub(2));
        let r0 = (v_top.floor() as usize).min(self.rows.saturating_sub(2));
        let (s, t) = ((u - c0 as f64).clamp(0.0, 1.0), (v_top - r0 as f64).clamp(0.0, 1.0));
        let (z00, z10) = (self.get(c0, r0)?, self.get(c0 + 1, r0)?);
        let (z01, z11) = (self.get(c0, r0 + 1)?, self.get(c0 + 1, r0 + 1)?);
        Some((1.0 - s) * (1.0 - t) * z00 + s * (1.0 - t) * z10 + (1.0 - s) * t * z01 + s * t * z11)
    }
}

#[derive(Default)]
struct AsciiHeader {
    cols: Option<usize>, rows: Option<usize>,
    xcorner: Option<f64>, ycorner: Option<f64>,
    xcenter: Option<f64>, ycenter: Option<f64>,
    cellsize: Option<f64>, nodata: Option<f64>,
}

impl AsciiHeader {
    fn resolve(self) -> Result<(usize, usize, f64, f64, f64, Option<f64>), GeoidError> {
        let cols = self.cols.ok_or(GeoidError::MissingHeader("ncols"))?;
        let rows = self.rows.ok_or(GeoidError::MissingHeader("nrows"))?;
        let step = self.cellsize.ok_or(GeoidError::MissingHeader("cellsize"))?;
        let x0 = self.xcorner.or_else(|| self.xcenter.map(|c| c - 0.5 * step))
            .ok_or(GeoidError::MissingHeader("xllcorner"))?;
        let y0 = self.ycorner.or_else(|| self.ycenter.map(|c| c - 0.5 * step))
            .ok_or(GeoidError::MissingHeader("yllcorner"))?;
        Ok((cols, rows, x0, y0, step, self.nodata))
    }
}
