//! EGM96 geoid height undulation grid interpolation.

/// Error conditions in EGM96 geoid grid operations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Egm96Error {
    /// Coordinate contains NaN or Infinity.
    NonFiniteCoordinate,
    /// Latitude lies outside [-90.0, 90.0] degrees.
    LatitudeOutOfRange(f64),
    /// Grid dimensions must have at least 2 rows and 2 columns.
    InvalidDimensions { cols: usize, rows: usize },
    /// Grid step size must be positive and finite.
    InvalidStep,
    /// Undulation buffer size does not match rows * cols.
    DataLengthMismatch { expected: usize, found: usize },
}

impl std::fmt::Display for Egm96Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteCoordinate => write!(f, "coordinates must be finite"),
            Self::LatitudeOutOfRange(lat) => {
                write!(f, "latitude {lat} out of range [-90, 90]")
            }
            Self::InvalidDimensions { cols, rows } => {
                write!(f, "grid dimensions must be >= 2x2, got {cols}x{rows}")
            }
            Self::InvalidStep => write!(f, "grid step must be positive and finite"),
            Self::DataLengthMismatch { expected, found } => {
                write!(f, "expected {expected} values, found {found}")
            }
        }
    }
}

impl std::error::Error for Egm96Error {}

/// Normalize longitude into standard [0.0, 360.0) range.
pub fn normalize_lon_360(lon_deg: f64) -> f64 {
    let rem = lon_deg % 360.0;
    if rem < 0.0 {
        rem + 360.0
    } else {
        rem
    }
}

/// Normalize longitude into standard [-180.0, 180.0) range.
pub fn normalize_lon_180(lon_deg: f64) -> f64 {
    let lon360 = normalize_lon_360(lon_deg);
    if lon360 >= 180.0 {
        lon360 - 360.0
    } else {
        lon360
    }
}

/// Regular geodetic grid for EGM96 geoid undulation interpolation.
#[derive(Debug, Clone, PartialEq)]
pub struct Egm96Geoid {
    pub min_lat_deg: f64,
    pub max_lat_deg: f64,
    pub lat_step_deg: f64,
    pub lon_step_deg: f64,
    pub cols: usize,
    pub rows: usize,
    pub undulations: Vec<f64>,
}

impl Egm96Geoid {
    /// Validate input dimensions and grid step sizes.
    fn validate_grid(
        lat_step: f64,
        lon_step: f64,
        cols: usize,
        rows: usize,
        data_len: usize,
    ) -> Result<(), Egm96Error> {
        if !lat_step.is_finite() || lat_step <= 0.0 || !lon_step.is_finite() || lon_step <= 0.0 {
            return Err(Egm96Error::InvalidStep);
        }
        if cols < 2 || rows < 2 {
            return Err(Egm96Error::InvalidDimensions { cols, rows });
        }
        let expected = cols * rows;
        if data_len != expected {
            return Err(Egm96Error::DataLengthMismatch { expected, found: data_len });
        }
        Ok(())
    }
}

impl Egm96Geoid {
    /// Construct a new global EGM96 geoid grid spanning [-90, +90] latitude.
    pub fn new(
        lat_step_deg: f64,
        lon_step_deg: f64,
        undulations: Vec<f64>,
    ) -> Result<Self, Egm96Error> {
        let rows = ((180.0 / lat_step_deg).round() as usize) + 1;
        let cols = (360.0 / lon_step_deg).round() as usize;
        Self::validate_grid(lat_step_deg, lon_step_deg, cols, rows, undulations.len())?;
        Ok(Self {
            min_lat_deg: -90.0,
            max_lat_deg: 90.0,
            lat_step_deg,
            lon_step_deg,
            cols,
            rows,
            undulations,
        })
    }
}

impl Egm96Geoid {
    /// Get undulation value at grid index (col, row) with periodic lon wrapping.
    #[inline]
    pub fn get_node(&self, col: usize, row: usize) -> f64 {
        let wrapped_col = col % self.cols;
        let clamped_row = row.min(self.rows - 1);
        self.undulations[clamped_row * self.cols + wrapped_col]
    }

    /// Check if coordinates are finite and within latitude limits.
    pub fn check_coords(&self, lat_deg: f64, lon_deg: f64) -> Result<(), Egm96Error> {
        if !lat_deg.is_finite() || !lon_deg.is_finite() {
            return Err(Egm96Error::NonFiniteCoordinate);
        }
        if !(-90.0..=90.0).contains(&lat_deg) {
            return Err(Egm96Error::LatitudeOutOfRange(lat_deg));
        }
        Ok(())
    }
}

impl Egm96Geoid {
    /// Compute geoid undulation N (in meters) via bilinear interpolation.
    pub fn undulation(&self, lat_deg: f64, lon_deg: f64) -> Result<f64, Egm96Error> {
        self.check_coords(lat_deg, lon_deg)?;
        let norm_lon = normalize_lon_360(lon_deg);
        let u = norm_lon / self.lon_step_deg;
        let v = (lat_deg - self.min_lat_deg) / self.lat_step_deg;
        let c0 = (u.floor() as usize) % self.cols;
        let c1 = (c0 + 1) % self.cols;
        let r0 = (v.floor() as usize).min(self.rows - 2);
        let r1 = r0 + 1;
        let s = (u - (u.floor())).clamp(0.0, 1.0);
        let t = (v - r0 as f64).clamp(0.0, 1.0);
        let z00 = self.get_node(c0, r0);
        let z10 = self.get_node(c1, r0);
        let z01 = self.get_node(c0, r1);
        let z11 = self.get_node(c1, r1);
        let top = (1.0 - s) * z00 + s * z10;
        let bottom = (1.0 - s) * z01 + s * z11;
        Ok((1.0 - t) * top + t * bottom)
    }
}

impl Egm96Geoid {
    /// Alias for undulation: geoid height N in meters.
    #[inline]
    pub fn geoid_height(&self, lat_deg: f64, lon_deg: f64) -> Result<f64, Egm96Error> {
        self.undulation(lat_deg, lon_deg)
    }

    /// Convert ellipsoidal height (HAE) to orthometric height (MSL): H = h - N.
    pub fn ellipsoidal_to_orthometric(
        &self,
        lat_deg: f64,
        lon_deg: f64,
        hae_m: f64,
    ) -> Result<f64, Egm96Error> {
        let n = self.undulation(lat_deg, lon_deg)?;
        Ok(hae_m - n)
    }
}

impl Egm96Geoid {
    /// Convert orthometric height (MSL) to ellipsoidal height (HAE): h = H + N.
    pub fn orthometric_to_ellipsoidal(
        &self,
        lat_deg: f64,
        lon_deg: f64,
        msl_m: f64,
    ) -> Result<f64, Egm96Error> {
        let n = self.undulation(lat_deg, lon_deg)?;
        Ok(msl_m + n)
    }
}

impl crate::geoid::GeoidModel for Egm96Geoid {
    fn undulation(&self, lat_deg: f64, lon_deg: f64) -> Option<f64> {
        self.undulation(lat_deg, lon_deg).ok()
    }
}

const COARSE_EGM96_SOUTH: [f64; 36] = [
    -30.0, -30.0, -30.0, -30.0, -30.0, -30.0, -30.0, -30.0, -30.0, -30.0, -30.0, -30.0,
    -10.0, 5.0, 12.0, -8.0, -25.0, 10.0, 18.0, 12.0, -5.0, -15.0, -20.0, -18.0,
    15.0, 18.0, -40.0, -60.0, 10.0, 30.0, 25.0, 18.0, 5.0, -10.0, -20.0, 5.0,
];

const COARSE_EGM96_NORTH: [f64; 48] = [
    17.0, 2.0, -15.0, -106.0, 15.0, 45.0, 20.0, 10.0, -5.0, -25.0, 15.0, 20.0,
    -30.0, -15.0, -35.0, -40.0, -20.0, 25.0, 10.0, -15.0, -30.0, -40.0, -10.0, -20.0,
    45.0, 30.0, -10.0, -25.0, -15.0, 10.0, 15.0, 5.0, -15.0, -30.0, -10.0, 25.0,
    14.0, 14.0, 14.0, 14.0, 14.0, 14.0, 14.0, 14.0, 14.0, 14.0, 14.0, 14.0,
];

impl Egm96Geoid {
    /// Create a coarse 30-degree EGM96 reference model for testing and fallback.
    pub fn coarse_reference_grid() -> Self {
        let mut data = Vec::with_capacity(84);
        data.extend_from_slice(&COARSE_EGM96_SOUTH);
        data.extend_from_slice(&COARSE_EGM96_NORTH);
        Self {
            min_lat_deg: -90.0,
            max_lat_deg: 90.0,
            lat_step_deg: 30.0,
            lon_step_deg: 30.0,
            cols: 12,
            rows: 7,
            undulations: data,
        }
    }
}
