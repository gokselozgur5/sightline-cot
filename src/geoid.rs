//! Geoid undulation models and height conversions.

/// Error conditions in geoid grid configuration, evaluation, and ASCII parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum GeoidError {
    InvalidDimensions,
    InvalidStep,
    InvalidBounds,
    NonFiniteCoordinate,
    DataLengthMismatch,
    EmptyInput,
    InvalidHeader(String),
    MissingHeader(&'static str),
    InvalidCellSize(f64),
    InvalidDataValue(String),
    DataCountMismatch { expected: usize, found: usize },
}

impl std::fmt::Display for GeoidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDimensions => write!(f, "grid dimensions must be at least 2x2"),
            Self::InvalidStep => write!(f, "step sizes must be positive and finite"),
            Self::InvalidBounds => write!(f, "grid bounds must lie within valid geodetic range"),
            Self::NonFiniteCoordinate => write!(f, "coordinates must be finite"),
            Self::DataLengthMismatch => write!(f, "undulation count must match cols * rows"),
            Self::EmptyInput => write!(f, "empty ascii grid input"),
            Self::InvalidHeader(m) => write!(f, "invalid ascii header: {m}"),
            Self::MissingHeader(h) => write!(f, "missing required header: {h}"),
            Self::InvalidCellSize(s) => write!(f, "invalid cell size: {s}"),
            Self::InvalidDataValue(v) => write!(f, "invalid float token: {v}"),
            Self::DataCountMismatch { expected, found } => {
                write!(f, "expected {expected} values, found {found}")
            }
        }
    }
}

impl std::error::Error for GeoidError {}

/// Trait for geoid separation models converting between heights.
pub trait GeoidModel {
    /// Compute geoid undulation (separation N in meters) at geodetic coordinates.
    fn undulation(&self, lat_deg: f64, lon_deg: f64) -> Option<f64>;

    /// Alias for undulation (separation N in meters).
    fn geoid_height(&self, lat_deg: f64, lon_deg: f64) -> Option<f64> {
        self.undulation(lat_deg, lon_deg)
    }

    /// Alias for undulation (separation N in meters).
    fn separation(&self, lat_deg: f64, lon_deg: f64) -> Option<f64> {
        self.undulation(lat_deg, lon_deg)
    }

    /// Convert orthometric height (H, MSL) to ellipsoidal height (h, HAE): h = H + N.
    fn orthometric_to_ellipsoidal(
        &self,
        lat_deg: f64,
        lon_deg: f64,
        orthometric_m: f64,
    ) -> Option<f64> {
        self.undulation(lat_deg, lon_deg).map(|n| orthometric_m + n)
    }

    /// Convert ellipsoidal height (h, HAE) to orthometric height (H, MSL): H = h - N.
    fn ellipsoidal_to_orthometric(
        &self,
        lat_deg: f64,
        lon_deg: f64,
        ellipsoidal_m: f64,
    ) -> Option<f64> {
        self.undulation(lat_deg, lon_deg).map(|n| ellipsoidal_m - n)
    }
}

impl<T: GeoidModel + ?Sized> GeoidModel for &T {
    fn undulation(&self, lat_deg: f64, lon_deg: f64) -> Option<f64> {
        (**self).undulation(lat_deg, lon_deg)
    }
}

impl<T: GeoidModel + ?Sized> GeoidModel for Box<T> {
    fn undulation(&self, lat_deg: f64, lon_deg: f64) -> Option<f64> {
        (**self).undulation(lat_deg, lon_deg)
    }
}

pub use GeoidModel as Geoid;

/// Uniform geoid model with constant undulation across valid coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstantGeoid {
    pub undulation_m: f64,
}

impl ConstantGeoid {
    pub const fn new(undulation_m: f64) -> Self {
        Self { undulation_m }
    }

    pub const fn zero() -> Self {
        Self { undulation_m: 0.0 }
    }
}

impl GeoidModel for ConstantGeoid {
    fn undulation(&self, lat_deg: f64, lon_deg: f64) -> Option<f64> {
        if !lat_deg.is_finite() || !lon_deg.is_finite() {
            return None;
        }
        if !(-90.0..=90.0).contains(&lat_deg) || !(-180.0..=360.0).contains(&lon_deg) {
            return None;
        }
        Some(self.undulation_m)
    }
}

/// Regular 2D geographic grid for bilinear geoid undulation interpolation.
#[derive(Debug, Clone, PartialEq)]
pub struct GeoidGrid {
    pub min_lat_deg: f64,
    pub min_lon_deg: f64,
    pub lat_step_deg: f64,
    pub lon_step_deg: f64,
    pub cols: usize,
    pub rows: usize,
    pub undulations: Vec<Option<f64>>,
}

pub type GeoidBilinearGrid = GeoidGrid;
pub type BilinearGeoidGrid = GeoidGrid;

impl GeoidGrid {
    fn validate_coords(lat0: f64, lon0: f64, dlat: f64, dlon: f64) -> Result<(), GeoidError> {
        if !lat0.is_finite() || !lon0.is_finite() {
            return Err(GeoidError::NonFiniteCoordinate);
        }
        if !dlat.is_finite() || dlat <= 0.0 || !dlon.is_finite() || dlon <= 0.0 {
            return Err(GeoidError::InvalidStep);
        }
        Ok(())
    }
}

impl GeoidGrid {
    fn validate(
        lat0: f64,
        lon0: f64,
        dlat: f64,
        dlon: f64,
        c: usize,
        r: usize,
        n: usize,
    ) -> Result<(), GeoidError> {
        Self::validate_coords(lat0, lon0, dlat, dlon)?;
        if c < 2 || r < 2 {
            return Err(GeoidError::InvalidDimensions);
        }
        if n != c * r {
            return Err(GeoidError::DataLengthMismatch);
        }
        let max_lat = lat0 + (r - 1) as f64 * dlat;
        if lat0 < -90.0 - 1e-9 || max_lat > 90.0 + 1e-9 {
            return Err(GeoidError::InvalidBounds);
        }
        Ok(())
    }
}

impl GeoidGrid {
    pub fn new(
        min_lat_deg: f64,
        min_lon_deg: f64,
        lat_step_deg: f64,
        lon_step_deg: f64,
        cols: usize,
        rows: usize,
        undulations: Vec<Option<f64>>,
    ) -> Result<Self, GeoidError> {
        Self::validate(
            min_lat_deg,
            min_lon_deg,
            lat_step_deg,
            lon_step_deg,
            cols,
            rows,
            undulations.len(),
        )?;
        Ok(Self {
            min_lat_deg,
            min_lon_deg,
            lat_step_deg,
            lon_step_deg,
            cols,
            rows,
            undulations,
        })
    }
}

impl GeoidGrid {
    pub fn new_uniform(
        min_lat_deg: f64,
        min_lon_deg: f64,
        step_deg: f64,
        cols: usize,
        rows: usize,
        undulations: Vec<Option<f64>>,
    ) -> Result<Self, GeoidError> {
        Self::new(
            min_lat_deg,
            min_lon_deg,
            step_deg,
            step_deg,
            cols,
            rows,
            undulations,
        )
    }
}

impl GeoidGrid {
    #[allow(clippy::too_many_arguments)]
    pub fn from_f64(
        min_lat_deg: f64,
        min_lon_deg: f64,
        lat_step_deg: f64,
        lon_step_deg: f64,
        cols: usize,
        rows: usize,
        raw: &[f64],
        nodata: Option<f64>,
    ) -> Result<Self, GeoidError> {
        let is_nd = |h: f64| !h.is_finite() || nodata.is_some_and(|nd| (h - nd).abs() < 1e-6);
        let undulations = raw.iter().map(|&h| (!is_nd(h)).then_some(h)).collect();
        Self::new(
            min_lat_deg,
            min_lon_deg,
            lat_step_deg,
            lon_step_deg,
            cols,
            rows,
            undulations,
        )
    }
}

impl GeoidGrid {
    pub fn from_f64_uniform(
        min_lat_deg: f64,
        min_lon_deg: f64,
        step_deg: f64,
        cols: usize,
        rows: usize,
        raw: &[f64],
        nodata: Option<f64>,
    ) -> Result<Self, GeoidError> {
        Self::from_f64(
            min_lat_deg,
            min_lon_deg,
            step_deg,
            step_deg,
            cols,
            rows,
            raw,
            nodata,
        )
    }
}

impl GeoidGrid {
    pub fn max_lat_deg(&self) -> f64 {
        self.min_lat_deg + (self.rows - 1) as f64 * self.lat_step_deg
    }

    pub fn max_lon_deg(&self) -> f64 {
        self.min_lon_deg + (self.cols - 1) as f64 * self.lon_step_deg
    }

    pub fn contains(&self, lat_deg: f64, lon_deg: f64) -> bool {
        if !lat_deg.is_finite() || !lon_deg.is_finite() {
            return false;
        }
        lat_deg >= self.min_lat_deg
            && lat_deg <= self.max_lat_deg()
            && lon_deg >= self.min_lon_deg
            && lon_deg <= self.max_lon_deg()
    }
}

impl GeoidGrid {
    pub fn get_node(&self, col: usize, row: usize) -> Option<f64> {
        if col < self.cols && row < self.rows {
            self.undulations[row * self.cols + col]
        } else {
            None
        }
    }
}

impl GeoidGrid {
    pub fn sample(&self, lat_deg: f64, lon_deg: f64) -> Option<f64> {
        if !self.contains(lat_deg, lon_deg) {
            return None;
        }
        let u = (lon_deg - self.min_lon_deg) / self.lon_step_deg;
        let v = (lat_deg - self.min_lat_deg) / self.lat_step_deg;
        crate::dem_bilinear::sample_bilinear_cells(u, v, self.cols, self.rows, false, |c, r| {
            self.get_node(c, r)
        })
    }
}

impl GeoidModel for GeoidGrid {
    fn undulation(&self, lat_deg: f64, lon_deg: f64) -> Option<f64> {
        self.sample(lat_deg, lon_deg)
    }
}

pub fn sample_geoid_bilinear(grid: &GeoidGrid, lat_deg: f64, lon_deg: f64) -> Option<f64> {
    grid.sample(lat_deg, lon_deg)
}

pub fn orthometric_to_ellipsoidal<G: GeoidModel + ?Sized>(
    model: &G,
    lat_deg: f64,
    lon_deg: f64,
    orthometric_m: f64,
) -> Option<f64> {
    model.orthometric_to_ellipsoidal(lat_deg, lon_deg, orthometric_m)
}

pub fn ellipsoidal_to_orthometric<G: GeoidModel + ?Sized>(
    model: &G,
    lat_deg: f64,
    lon_deg: f64,
    ellipsoidal_m: f64,
) -> Option<f64> {
    model.ellipsoidal_to_orthometric(lat_deg, lon_deg, ellipsoidal_m)
}
