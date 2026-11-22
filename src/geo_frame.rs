//! Geodetic LocalFrame with cached reference trigonometry.

/// Error indicating failure during LocalFrame creation or projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeoFrameError {
    /// Non-finite coordinate or parameter value encountered.
    NonFinite,
    /// Latitude outside the valid range [-90, 90] degrees.
    LatitudeOutOfRange,
}

impl std::fmt::Display for GeoFrameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFinite => write!(f, "non-finite coordinate or parameter"),
            Self::LatitudeOutOfRange => write!(f, "latitude must be between -90 and 90 degrees"),
        }
    }
}

impl std::error::Error for GeoFrameError {}

use crate::geo_ecef::{ecef_to_geodetic, geodetic_to_ecef};

/// Local tangent frame caching reference geodetic coordinates, trigonometry, and ECEF origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocalFrame {
    pub origin_lat_deg: f64,
    pub origin_lon_deg: f64,
    pub origin_hae_m: f64,
    pub sin_lat: f64,
    pub cos_lat: f64,
    pub sin_lon: f64,
    pub cos_lon: f64,
    pub origin_ecef: [f64; 3],
}

impl LocalFrame {
    /// Construct a new LocalFrame caching reference trigonometry and ECEF coordinates.
    pub fn new(lat_deg: f64, lon_deg: f64, hae_m: f64) -> Result<Self, GeoFrameError> {
        if !lat_deg.is_finite() || !lon_deg.is_finite() || !hae_m.is_finite() {
            return Err(GeoFrameError::NonFinite);
        }
        if !(-90.0..=90.0).contains(&lat_deg) {
            return Err(GeoFrameError::LatitudeOutOfRange);
        }
        let lat_rad = lat_deg.to_radians();
        let lon_rad = lon_deg.to_radians();
        let (sin_lat, cos_lat) = (lat_rad.sin(), lat_rad.cos());
        let (sin_lon, cos_lon) = (lon_rad.sin(), lon_rad.cos());
        let origin_ecef = geodetic_to_ecef(lat_deg, lon_deg, hae_m);
        Ok(Self {
            origin_lat_deg: lat_deg,
            origin_lon_deg: lon_deg,
            origin_hae_m: hae_m,
            sin_lat,
            cos_lat,
            sin_lon,
            cos_lon,
            origin_ecef,
        })
    }

    /// Convert an ECEF coordinate to local ENU offset using cached frame parameters.
    pub fn ecef_to_enu(&self, p_ecef: [f64; 3]) -> Result<[f64; 3], GeoFrameError> {
        if !p_ecef[0].is_finite() || !p_ecef[1].is_finite() || !p_ecef[2].is_finite() {
            return Err(GeoFrameError::NonFinite);
        }
        let dx = p_ecef[0] - self.origin_ecef[0];
        let dy = p_ecef[1] - self.origin_ecef[1];
        let dz = p_ecef[2] - self.origin_ecef[2];
        let e = -self.sin_lon * dx + self.cos_lon * dy;
        let n = -self.sin_lat * self.cos_lon * dx - self.sin_lat * self.sin_lon * dy
            + self.cos_lat * dz;
        let u =
            self.cos_lat * self.cos_lon * dx + self.cos_lat * self.sin_lon * dy + self.sin_lat * dz;
        Ok([e, n, u])
    }

    /// Convert geodetic coordinates to local ENU offset in meters.
    pub fn geodetic_to_enu(
        &self,
        lat_deg: f64,
        lon_deg: f64,
        hae_m: f64,
    ) -> Result<[f64; 3], GeoFrameError> {
        if !lat_deg.is_finite() || !lon_deg.is_finite() || !hae_m.is_finite() {
            return Err(GeoFrameError::NonFinite);
        }
        if !(-90.0..=90.0).contains(&lat_deg) {
            return Err(GeoFrameError::LatitudeOutOfRange);
        }
        let p_ecef = geodetic_to_ecef(lat_deg, lon_deg, hae_m);
        self.ecef_to_enu(p_ecef)
    }

    /// Convert local ENU offset in meters to ECEF coordinates.
    pub fn enu_to_ecef(&self, enu_m: [f64; 3]) -> Result<[f64; 3], GeoFrameError> {
        if !enu_m[0].is_finite() || !enu_m[1].is_finite() || !enu_m[2].is_finite() {
            return Err(GeoFrameError::NonFinite);
        }
        let (e, n, u) = (enu_m[0], enu_m[1], enu_m[2]);
        let dx =
            -self.sin_lon * e - self.sin_lat * self.cos_lon * n + self.cos_lat * self.cos_lon * u;
        let dy =
            self.cos_lon * e - self.sin_lat * self.sin_lon * n + self.cos_lat * self.sin_lon * u;
        let dz = self.cos_lat * n + self.sin_lat * u;
        Ok([
            self.origin_ecef[0] + dx,
            self.origin_ecef[1] + dy,
            self.origin_ecef[2] + dz,
        ])
    }

    /// Convert local ENU offset in meters to geodetic coordinates (lat, lon, hae).
    pub fn enu_to_geodetic(&self, enu_m: [f64; 3]) -> Result<(f64, f64, f64), GeoFrameError> {
        let ecef = self.enu_to_ecef(enu_m)?;
        Ok(ecef_to_geodetic(ecef[0], ecef[1], ecef[2]))
    }

    /// Batch project geodetic coordinates `[lat_deg, lon_deg, hae_m]` to ENU offsets.
    pub fn geodetic_to_enu_batch(
        &self,
        coords: &[[f64; 3]],
    ) -> Result<Vec<[f64; 3]>, GeoFrameError> {
        let mut out = Vec::with_capacity(coords.len());
        for c in coords {
            out.push(self.geodetic_to_enu(c[0], c[1], c[2])?);
        }
        Ok(out)
    }

    /// Batch convert local ENU offsets `[e, n, u]` to geodetic coordinates.
    pub fn enu_to_geodetic_batch(
        &self,
        enu_coords: &[[f64; 3]],
    ) -> Result<Vec<(f64, f64, f64)>, GeoFrameError> {
        let mut out = Vec::with_capacity(enu_coords.len());
        for enu in enu_coords {
            out.push(self.enu_to_geodetic(*enu)?);
        }
        Ok(out)
    }

    /// Project 2D coordinates (lat_deg, lon_deg at hae = 0) to local [east_m, north_m].
    pub fn project_2d(&self, lat_deg: f64, lon_deg: f64) -> Result<[f64; 2], GeoFrameError> {
        let enu = self.geodetic_to_enu(lat_deg, lon_deg, 0.0)?;
        Ok([enu[0], enu[1]])
    }

    /// Batch project 2D coordinates `[lat_deg, lon_deg]` to `[east_m, north_m]`.
    pub fn project_2d_batch(&self, pts: &[[f64; 2]]) -> Result<Vec<[f64; 2]>, GeoFrameError> {
        let mut out = Vec::with_capacity(pts.len());
        for p in pts {
            out.push(self.project_2d(p[0], p[1])?);
        }
        Ok(out)
    }

    /// Unproject 2D planar coordinates [east_m, north_m] to (lat_deg, lon_deg).
    pub fn unproject_2d(&self, east_m: f64, north_m: f64) -> Result<(f64, f64), GeoFrameError> {
        let (lat, lon, _) = self.enu_to_geodetic([east_m, north_m, 0.0])?;
        Ok((lat, lon))
    }
}
