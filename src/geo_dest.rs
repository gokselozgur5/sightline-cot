//! Geodesic destination point and distance computations.

/// Errors that can occur during geodesic destination calculation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeoDestError {
    InvalidLatitude,
    InvalidLongitude,
    NegativeRange,
    NonFiniteInput,
}

impl core::fmt::Display for GeoDestError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLatitude => write!(f, "latitude out of range [-90, 90]"),
            Self::InvalidLongitude => write!(f, "longitude out of range [-180, 180]"),
            Self::NegativeRange => write!(f, "range cannot be negative"),
            Self::NonFiniteInput => write!(f, "input values must be finite"),
        }
    }
}

impl std::error::Error for GeoDestError {}

/// Mean Earth radius in metres based on the WGS84 ellipsoid.
pub const WGS84_MEAN_RADIUS_M: f64 = 6_371_008.8;

/// Computes great-circle distance between two points in metres.
pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let phi1 = lat1.to_radians();
    let phi2 = lat2.to_radians();
    let dphi = (lat2 - lat1).to_radians();
    let dlam = (lon2 - lon1).to_radians();
    let a = (dphi / 2.0).sin().powi(2)
        + phi1.cos() * phi2.cos() * (dlam / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt().max(0.0));
    WGS84_MEAN_RADIUS_M * c
}

/// Computes the initial bearing from point 1 to point 2 in degrees [0, 360).
pub fn initial_bearing(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let phi1 = lat1.to_radians();
    let phi2 = lat2.to_radians();
    let dlam = (lon2 - lon1).to_radians();
    let y = dlam.sin() * phi2.cos();
    let x = phi1.cos() * phi2.sin() - phi1.sin() * phi2.cos() * dlam.cos();
    let b = y.atan2(x).to_degrees();
    (b + 360.0).rem_euclid(360.0)
}

fn validate_inputs(
    lat_deg: f64,
    lon_deg: f64,
    bearing_deg: f64,
    range_m: f64,
    radius_m: f64,
) -> Result<(), GeoDestError> {
    if !lat_deg.is_finite()
        || !lon_deg.is_finite()
        || !bearing_deg.is_finite()
        || !range_m.is_finite()
        || !radius_m.is_finite()
    {
        return Err(GeoDestError::NonFiniteInput);
    }
    if range_m < 0.0 || radius_m <= 0.0 {
        return Err(GeoDestError::NegativeRange);
    }
    if !(-90.0..=90.0).contains(&lat_deg) {
        return Err(GeoDestError::InvalidLatitude);
    }
    if !(-180.0..=180.0).contains(&lon_deg) {
        return Err(GeoDestError::InvalidLongitude);
    }
    Ok(())
}
