//! Vincenty direct and inverse geodesic algorithms on the WGS84 ellipsoid.

/// Errors from Vincenty geodesic computations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VincentyError {
    InvalidLatitude,
    InvalidLongitude,
    NegativeDistance,
    NonFiniteInput,
    NoConvergence,
}

impl core::fmt::Display for VincentyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLatitude => write!(f, "latitude out of range [-90, 90]"),
            Self::InvalidLongitude => write!(f, "longitude out of range [-180, 180]"),
            Self::NegativeDistance => write!(f, "distance cannot be negative"),
            Self::NonFiniteInput => write!(f, "input values must be finite"),
            Self::NoConvergence => write!(f, "Vincenty formula failed to converge"),
        }
    }
}

impl std::error::Error for VincentyError {}

/// Result of the Vincenty inverse geodesic calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VincentyInverse {
    pub distance_m: f64,
    pub forward_azimuth_deg: f64,
    pub reverse_azimuth_deg: f64,
}

/// Result of the Vincenty direct geodesic calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VincentyDirect {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub reverse_azimuth_deg: f64,
}

pub const WGS84_A: f64 = 6_378_137.0;
pub const WGS84_F: f64 = 1.0 / 298.257_223_563;
pub const WGS84_B: f64 = 6_356_752.314_245_179;

fn validate_point(lat_deg: f64, lon_deg: f64) -> Result<(), VincentyError> {
    if !lat_deg.is_finite() || !lon_deg.is_finite() {
        return Err(VincentyError::NonFiniteInput);
    }
    if !(-90.0..=90.0).contains(&lat_deg) {
        return Err(VincentyError::InvalidLatitude);
    }
    if !(-180.0..=180.0).contains(&lon_deg) {
        return Err(VincentyError::InvalidLongitude);
    }
    Ok(())
}

fn vincenty_ab(cos2_alpha: f64) -> (f64, f64) {
    let u2 = cos2_alpha * (WGS84_A * WGS84_A - WGS84_B * WGS84_B) / (WGS84_B * WGS84_B);
    let a = 1.0 + u2 / 16384.0 * (4096.0 + u2 * (-768.0 + u2 * (320.0 - 175.0 * u2)));
    let b = u2 / 1024.0 * (256.0 + u2 * (-128.0 + u2 * (74.0 - 47.0 * u2)));
    (a, b)
}

fn vincenty_c(cos2_alpha: f64) -> f64 {
    WGS84_F / 16.0 * cos2_alpha * (4.0 + WGS84_F * (4.0 - 3.0 * cos2_alpha))
}

fn vincenty_delta_sigma(b: f64, sin_sigma: f64, cos_sigma: f64, cos_2sigma_m: f64) -> f64 {
    let cos2_2sigma_m = cos_2sigma_m * cos_2sigma_m;
    b * sin_sigma
        * (cos_2sigma_m
            + b / 4.0
                * (cos_sigma * (-1.0 + 2.0 * cos2_2sigma_m)
                    - b / 6.0
                        * cos_2sigma_m
                        * (-3.0 + 4.0 * sin_sigma * sin_sigma)
                        * (-3.0 + 4.0 * cos2_2sigma_m)))
}

fn solve_step(
    sin_u1: f64,
    cos_u1: f64,
    sin_u2: f64,
    cos_u2: f64,
    lambda: f64,
) -> Result<(f64, f64, f64, f64, f64, f64), VincentyError> {
    let (sin_l, cos_l) = (lambda.sin(), lambda.cos());
    let sin_sigma =
        ((cos_u2 * sin_l).powi(2) + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_l).powi(2)).sqrt();
    if sin_sigma.abs() < 1e-12 {
        return Err(VincentyError::NoConvergence);
    }
    let cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_l;
    let sigma = sin_sigma.atan2(cos_sigma);
    let sin_alpha = cos_u1 * cos_u2 * sin_l / sin_sigma;
    let cos2_alpha = 1.0 - sin_alpha * sin_alpha;
    let cos_2sigma_m = if cos2_alpha.abs() > 1e-12 {
        cos_sigma - 2.0 * sin_u1 * sin_u2 / cos2_alpha
    } else {
        0.0
    };
    Ok((sigma, sin_sigma, cos_sigma, sin_alpha, cos2_alpha, cos_2sigma_m))
}
