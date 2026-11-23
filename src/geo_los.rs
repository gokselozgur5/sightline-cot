//! Line-of-sight elevation and slant-range calculator.

/// Errors occurring during line-of-sight calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeoLosError {
    NonFiniteCoordinate,
    LatitudeOutOfRange,
    CoincidentPoints,
}

impl core::fmt::Display for GeoLosError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFiniteCoordinate => write!(f, "coordinate is non-finite"),
            Self::LatitudeOutOfRange => write!(f, "latitude must be between -90 and 90 degrees"),
            Self::CoincidentPoints => write!(f, "observer and target positions are coincident"),
        }
    }
}

impl std::error::Error for GeoLosError {}

/// 3D geographic point specified by latitude, longitude, and height above ellipsoid.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoPoint3D {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub hae_m: f64,
}

impl GeoPoint3D {
    /// Create and validate a geographic 3D position.
    pub fn new(lat_deg: f64, lon_deg: f64, hae_m: f64) -> Result<Self, GeoLosError> {
        if !lat_deg.is_finite() || !lon_deg.is_finite() || !hae_m.is_finite() {
            return Err(GeoLosError::NonFiniteCoordinate);
        }
        if !(-90.0..=90.0).contains(&lat_deg) {
            return Err(GeoLosError::LatitudeOutOfRange);
        }
        Ok(Self {
            lat_deg,
            lon_deg,
            hae_m,
        })
    }
}

/// Result of a 3D line-of-sight calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LosGeometry {
    pub slant_range_m: f64,
    pub ground_range_m: f64,
    pub elevation_angle_rad: f64,
    pub azimuth_rad: f64,
    pub horizon_distance_m: f64,
    pub is_direct_los: bool,
}

impl LosGeometry {
    /// Return the elevation angle in degrees.
    pub fn elevation_angle_deg(&self) -> f64 {
        self.elevation_angle_rad.to_degrees()
    }

    /// Return the azimuth (true bearing) in degrees in range [0, 360).
    pub fn azimuth_deg(&self) -> f64 {
        let deg = self.azimuth_rad.to_degrees();
        (deg % 360.0 + 360.0) % 360.0
    }
}

pub use crate::los::WGS84_EARTH_RADIUS_M as WGS84_MEAN_RADIUS_M;

/// Compute geometric horizon line-of-sight limit distance between two heights above ground.
///
/// Uses geometric propagation (k = 0) with standard spherical earth radius WGS84_MEAN_RADIUS_M.
pub fn horizon_distance_m(h1_m: f64, h2_m: f64) -> Result<f64, GeoLosError> {
    if !h1_m.is_finite() || !h2_m.is_finite() {
        return Err(GeoLosError::NonFiniteCoordinate);
    }
    let d1 = if h1_m > 0.0 {
        (2.0 * WGS84_MEAN_RADIUS_M * h1_m + h1_m * h1_m).sqrt()
    } else {
        0.0
    };
    let d2 = if h2_m > 0.0 {
        (2.0 * WGS84_MEAN_RADIUS_M * h2_m + h2_m * h2_m).sqrt()
    } else {
        0.0
    };
    Ok(d1 + d2)
}

use crate::egm96_geoid::Egm96Geoid;
use crate::geo_enu::geodetic_to_enu;

/// Compute line-of-sight slant range, elevation, azimuth and horizon clearance.
pub fn compute_geo_los(obs: &GeoPoint3D, target: &GeoPoint3D) -> Result<LosGeometry, GeoLosError> {
    let enu = geodetic_to_enu(
        target.lat_deg,
        target.lon_deg,
        target.hae_m,
        obs.lat_deg,
        obs.lon_deg,
        obs.hae_m,
    )
    .map_err(|_| GeoLosError::NonFiniteCoordinate)?;
    let (e, n, u) = (enu[0], enu[1], enu[2]);
    let ground_range_m = (e * e + n * n).sqrt();
    let slant_range_m = (ground_range_m * ground_range_m + u * u).sqrt();
    if slant_range_m < 1e-6 {
        return Err(GeoLosError::CoincidentPoints);
    }
    let elevation_angle_rad = u.atan2(ground_range_m);
    let az_raw = e.atan2(n);
    let azimuth_rad = if az_raw < 0.0 {
        az_raw + 2.0 * std::f64::consts::PI
    } else {
        az_raw
    };
    let geoid = Egm96Geoid::coarse_reference_grid();
    let n1 = geoid.undulation(obs.lat_deg, obs.lon_deg).unwrap_or(0.0);
    let n2 = geoid
        .undulation(target.lat_deg, target.lon_deg)
        .unwrap_or(0.0);
    // In areas with deep geoid depressions (such as the Indian Ocean geoid low,
    // where undulation reaches ~-107 m), coarse reference grids may underestimate
    // the negative undulation. When HAE is below the coarse geoid, bound the local
    // surface reference to the physical global minimum geoid undulation (-107 m).
    const MIN_GEOID_UNDULATION_M: f64 = -107.0;
    let ref1 = if obs.hae_m < n1 {
        n1.min(MIN_GEOID_UNDULATION_M)
    } else {
        n1
    };
    let ref2 = if target.hae_m < n2 {
        n2.min(MIN_GEOID_UNDULATION_M)
    } else {
        n2
    };
    let h1_m = (obs.hae_m - ref1).max(0.0);
    let h2_m = (target.hae_m - ref2).max(0.0);
    let horiz_m = horizon_distance_m(h1_m, h2_m)?;
    let is_direct_los = ground_range_m <= horiz_m;
    Ok(LosGeometry {
        slant_range_m,
        ground_range_m,
        elevation_angle_rad,
        azimuth_rad,
        horizon_distance_m: horiz_m,
        is_direct_los,
    })
}

/// Compute 3D slant range distance between observer and target in meters.
pub fn slant_range_geodetic(obs: &GeoPoint3D, target: &GeoPoint3D) -> Result<f64, GeoLosError> {
    compute_geo_los(obs, target).map(|geom| geom.slant_range_m)
}

/// Compute elevation angle in radians from observer to target.
pub fn elevation_angle_rad(obs: &GeoPoint3D, target: &GeoPoint3D) -> Result<f64, GeoLosError> {
    compute_geo_los(obs, target).map(|geom| geom.elevation_angle_rad)
}

/// Determine whether target is within unobstructed geometric line of sight.
pub fn has_line_of_sight(obs: &GeoPoint3D, target: &GeoPoint3D) -> Result<bool, GeoLosError> {
    compute_geo_los(obs, target).map(|geom| geom.is_direct_los)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_validation() {
        assert_eq!(
            GeoPoint3D::new(f64::NAN, 10.0, 100.0),
            Err(GeoLosError::NonFiniteCoordinate)
        );
        assert_eq!(
            GeoPoint3D::new(91.0, 10.0, 100.0),
            Err(GeoLosError::LatitudeOutOfRange)
        );
        assert_eq!(
            GeoPoint3D::new(-90.1, 10.0, 100.0),
            Err(GeoLosError::LatitudeOutOfRange)
        );
        assert!(GeoPoint3D::new(45.0, 10.0, 100.0).is_ok());
    }

    #[test]
    fn test_coincident_positions_error() {
        let p = GeoPoint3D::new(37.5, -122.0, 50.0).unwrap();
        assert_eq!(compute_geo_los(&p, &p), Err(GeoLosError::CoincidentPoints));
    }

    #[test]
    fn test_vertical_los_elevation() {
        let obs = GeoPoint3D::new(0.0, 0.0, 100.0).unwrap();
        let target = GeoPoint3D::new(0.0, 0.0, 1100.0).unwrap();
        let geom = compute_geo_los(&obs, &target).unwrap();
        assert!((geom.slant_range_m - 1000.0).abs() < 1e-2);
        assert!(geom.ground_range_m.abs() < 1e-2);
        assert!((geom.elevation_angle_deg() - 90.0).abs() < 1e-3);
        assert!(geom.is_direct_los);
    }

    #[test]
    fn test_cardinal_azimuth_and_range() {
        let obs = GeoPoint3D::new(0.0, 0.0, 0.0).unwrap();
        let north_target = GeoPoint3D::new(0.01, 0.0, 0.0).unwrap();
        let geom_n = compute_geo_los(&obs, &north_target).unwrap();
        assert!(geom_n.azimuth_deg().abs() < 1e-2);
        assert!(geom_n.elevation_angle_deg() < 0.0);

        let east_target = GeoPoint3D::new(0.0, 0.01, 0.0).unwrap();
        let geom_e = compute_geo_los(&obs, &east_target).unwrap();
        assert!((geom_e.azimuth_deg() - 90.0).abs() < 1e-2);
    }

    #[test]
    fn test_horizon_distance_and_visibility() {
        let d = horizon_distance_m(100.0, 0.0).unwrap();
        let expected = (2.0 * WGS84_MEAN_RADIUS_M * 100.0 + 10000.0).sqrt();
        assert!((d - expected).abs() < 1.0);

        let obs = GeoPoint3D::new(0.0, 0.0, 30.0).unwrap();
        let close_target = GeoPoint3D::new(0.01, 0.0, 30.0).unwrap();
        assert!(has_line_of_sight(&obs, &close_target).unwrap());
    }
}
