//! Geoid undulation models and height conversions.

/// Error conditions in geoid grid configuration and evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeoidError {
    InvalidDimensions,
    InvalidStep,
    InvalidBounds,
    NonFiniteCoordinate,
    DataLengthMismatch,
}

impl std::fmt::Display for GeoidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDimensions => write!(f, "grid dimensions must be at least 2x2"),
            Self::InvalidStep => write!(f, "step sizes must be positive and finite"),
            Self::InvalidBounds => write!(f, "grid bounds must lie within valid geodetic range"),
            Self::NonFiniteCoordinate => write!(f, "coordinates must be finite"),
            Self::DataLengthMismatch => write!(f, "undulation count must match cols * rows"),
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
    fn orthometric_to_ellipsoidal(&self, lat_deg: f64, lon_deg: f64, orthometric_m: f64) -> Option<f64> {
        self.undulation(lat_deg, lon_deg).map(|n| orthometric_m + n)
    }

    /// Convert ellipsoidal height (h, HAE) to orthometric height (H, MSL): H = h - N.
    fn ellipsoidal_to_orthometric(&self, lat_deg: f64, lon_deg: f64, ellipsoidal_m: f64) -> Option<f64> {
        self.undulation(lat_deg, lon_deg).map(|n| ellipsoidal_m - n)
    }
}
