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
        let expected = cols
            .checked_mul(rows)
            .ok_or(Egm96Error::InvalidDimensions { cols, rows })?;
        if data_len != expected {
            return Err(Egm96Error::DataLengthMismatch {
                expected,
                found: data_len,
            });
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
        if !lat_step_deg.is_finite()
            || lat_step_deg <= 0.0
            || !lon_step_deg.is_finite()
            || lon_step_deg <= 0.0
        {
            return Err(Egm96Error::InvalidStep);
        }
        let rows = ((180.0 / lat_step_deg).round() as usize)
            .checked_add(1)
            .ok_or(Egm96Error::InvalidDimensions {
                cols: (360.0 / lon_step_deg).round() as usize,
                rows: usize::MAX,
            })?;
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
        crate::dem_bilinear::sample_bilinear_cells(u, v, self.cols, self.rows, true, |c, r| {
            Some(self.get_node(c, r))
        })
        .ok_or(Egm96Error::NonFiniteCoordinate)
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
    -29.53, -29.53, -29.53, -29.53, -29.53, -29.53, -29.53, -29.53, -29.53, -29.53, -29.53, -29.53,
    15.13, 28.91, 32.55, 8.88, -24.63, -29.65, -46.3, -32.4, -23.65, -11.12, 20.2, 23.54, 21.41,
    30.21, 14.92, -25.2, -22.6, 31.69, 47.53, -1.44, -10.46, 0.86, 17.79, 3.49,
];

const COARSE_EGM96_NORTH: [f64; 48] = [
    17.16, -9.82, -62.75, -63.24, 58.35, 63.32, 21.15, 12.73, -23.15, -4.29, -12.63, 2.16, 35.39,
    16.21, -15.06, -34.08, 7.02, 19.33, -7.5, -15.84, -42.4, -27.26, -40.79, 30.19, 48.23, 15.99,
    -3.87, -33.91, -16.61, 16.21, 0.44, 13.46, -15.9, -47.04, 4.82, 63.8, 13.61, 13.61, 13.61,
    13.61, 13.61, 13.61, 13.61, 13.61, 13.61, 13.61, 13.61, 13.61,
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

/// Interpolate EGM96 geoid undulation from a given geoid grid.
pub fn sample_egm96_undulation(
    geoid: &Egm96Geoid,
    lat_deg: f64,
    lon_deg: f64,
) -> Result<f64, Egm96Error> {
    geoid.undulation(lat_deg, lon_deg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_lon_360() {
        assert_eq!(normalize_lon_360(0.0), 0.0);
        assert_eq!(normalize_lon_360(360.0), 0.0);
        assert_eq!(normalize_lon_360(-90.0), 270.0);
        assert_eq!(normalize_lon_360(450.0), 90.0);
        assert_eq!(normalize_lon_180(0.0), 0.0);
        assert_eq!(normalize_lon_180(270.0), -90.0);
        assert_eq!(normalize_lon_180(-180.0), -180.0);
    }

    #[test]
    fn test_grid_validation_errors() {
        let err = Egm96Geoid::new(-1.0, 30.0, vec![0.0; 84]);
        assert_eq!(err, Err(Egm96Error::InvalidStep));
        let err2 = Egm96Geoid::new(30.0, 30.0, vec![0.0; 10]);
        assert_eq!(
            err2,
            Err(Egm96Error::DataLengthMismatch {
                expected: 84,
                found: 10
            })
        );
        let grid = Egm96Geoid::coarse_reference_grid();
        assert_eq!(
            grid.undulation(95.0, 0.0),
            Err(Egm96Error::LatitudeOutOfRange(95.0))
        );
        assert_eq!(
            grid.undulation(f64::NAN, 0.0),
            Err(Egm96Error::NonFiniteCoordinate)
        );
    }

    #[test]
    fn test_exact_node_undulation() {
        let grid = Egm96Geoid::coarse_reference_grid();
        let n_south_pole = grid.undulation(-90.0, 0.0).unwrap();
        assert!((n_south_pole - (-29.53)).abs() < 1e-6);
        let n_north_pole = grid.undulation(90.0, 180.0).unwrap();
        assert!((n_north_pole - 13.61).abs() < 1e-6);
        let n_ind_ocean = grid.undulation(0.0, 90.0).unwrap();
        assert!((n_ind_ocean - (-63.24)).abs() < 1e-6);
    }

    #[test]
    fn test_bilinear_interpolation_midpoint() {
        let grid = Egm96Geoid::coarse_reference_grid();
        let n_interp = grid.undulation(75.0, 15.0).unwrap();
        assert!((n_interp - 22.86).abs() < 1e-4);
    }

    #[test]
    fn test_antimeridian_and_wrap() {
        let grid = Egm96Geoid::coarse_reference_grid();
        let n1 = grid.undulation(0.0, 350.0).unwrap();
        let n2 = grid.undulation(0.0, -10.0).unwrap();
        assert!((n1 - n2).abs() < 1e-6);
    }

    #[test]
    fn test_height_conversions() {
        let grid = Egm96Geoid::coarse_reference_grid();
        let (lat, lon) = (0.0, 90.0);
        let h_ellips = 100.0;
        let h_ortho = grid.ellipsoidal_to_orthometric(lat, lon, h_ellips).unwrap();
        assert!((h_ortho - 163.24).abs() < 1e-6);
        let roundtrip = grid.orthometric_to_ellipsoidal(lat, lon, h_ortho).unwrap();
        assert!((roundtrip - h_ellips).abs() < 1e-6);
    }

    #[test]
    fn test_geoid_model_trait_integration() {
        use crate::geoid::GeoidModel;
        let grid = Egm96Geoid::coarse_reference_grid();
        let undulation = GeoidModel::undulation(&grid, 0.0, 90.0);
        assert_eq!(undulation, Some(-63.24));
    }
}
