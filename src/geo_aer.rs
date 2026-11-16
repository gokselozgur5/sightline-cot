//! Geocentric to Topocentric (SEZ / AER) coordinate transformer.

/// Error indicating failure during topocentric conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AerError {
    /// Non-finite coordinate or parameter value.
    NonFinite,
    /// Latitude outside the valid range [-90, 90] degrees.
    LatitudeOutOfRange,
    /// Slant range is negative.
    NegativeRange,
    /// Elevation angle outside [-90, 90] degrees.
    ElevationOutOfRange,
}

impl std::error::Error for AerError {}

impl std::fmt::Display for AerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFinite => write!(f, "non-finite coordinate or parameter"),
            Self::LatitudeOutOfRange => write!(f, "latitude must be between -90 and 90 degrees"),
            Self::NegativeRange => write!(f, "slant range cannot be negative"),
            Self::ElevationOutOfRange => write!(f, "elevation must be between -90 and 90 degrees"),
        }
    }
}

/// Topocentric Azimuth, Elevation, Range (AER) coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AerCoord {
    /// Azimuth angle measured clockwise from True North in degrees [0, 360).
    pub azimuth_deg: f64,
    /// Elevation angle above local horizon in degrees [-90, 90].
    pub elevation_deg: f64,
    /// Slant range distance in meters (>= 0).
    pub range_m: f64,
}

/// Topocentric South, East, Zenith (SEZ) coordinates in meters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SezCoord {
    /// Southward displacement in meters along local meridian.
    pub south_m: f64,
    /// Eastward displacement in meters along local parallel.
    pub east_m: f64,
    /// Zenith displacement in meters normal to reference ellipsoid.
    pub zenith_m: f64,
}

impl AerCoord {
    /// Create and validate a new AER coordinate.
    pub fn new(azimuth_deg: f64, elevation_deg: f64, range_m: f64) -> Result<Self, AerError> {
        if !azimuth_deg.is_finite() || !elevation_deg.is_finite() || !range_m.is_finite() {
            return Err(AerError::NonFinite);
        }
        if !(-90.0..=90.0).contains(&elevation_deg) {
            return Err(AerError::ElevationOutOfRange);
        }
        if range_m < 0.0 {
            return Err(AerError::NegativeRange);
        }
        let norm_az = (azimuth_deg % 360.0 + 360.0) % 360.0;
        let final_az = if (norm_az - 360.0).abs() < 1e-12 { 0.0 } else { norm_az };
        Ok(Self { azimuth_deg: final_az, elevation_deg, range_m })
    }
}

impl SezCoord {
    /// Create and validate a new SEZ coordinate.
    pub fn new(south_m: f64, east_m: f64, zenith_m: f64) -> Result<Self, AerError> {
        if !south_m.is_finite() || !east_m.is_finite() || !zenith_m.is_finite() {
            return Err(AerError::NonFinite);
        }
        Ok(Self { south_m, east_m, zenith_m })
    }

    /// Return SEZ components as an array [south_m, east_m, zenith_m].
    pub fn to_array(&self) -> [f64; 3] {
        [self.south_m, self.east_m, self.zenith_m]
    }
}

/// Convert local ENU (East, North, Up) vector to SEZ (South, East, Zenith).
#[inline]
pub fn enu_to_sez(enu: [f64; 3]) -> SezCoord {
    SezCoord {
        south_m: -enu[1],
        east_m: enu[0],
        zenith_m: enu[2],
    }
}

/// Convert local SEZ (South, East, Zenith) vector to ENU (East, North, Up).
#[inline]
pub fn sez_to_enu(sez: SezCoord) -> [f64; 3] {
    [sez.east_m, -sez.south_m, sez.zenith_m]
}

/// Convert topocentric SEZ coordinates to AER (Azimuth, Elevation, Range).
pub fn sez_to_aer(sez: SezCoord) -> Result<AerCoord, AerError> {
    let (s, e, z) = (sez.south_m, sez.east_m, sez.zenith_m);
    let range = (s * s + e * e + z * z).sqrt();
    if range < 1e-12 {
        return AerCoord::new(0.0, 0.0, 0.0);
    }
    let horiz = (s * s + e * e).sqrt();
    let el_deg = z.atan2(horiz).to_degrees();
    let mut az_deg = e.atan2(-s).to_degrees();
    if az_deg < 0.0 {
        az_deg += 360.0;
    }
    AerCoord::new(az_deg, el_deg, range)
}

/// Convert topocentric AER coordinates to SEZ (South, East, Zenith).
pub fn aer_to_sez(aer: AerCoord) -> SezCoord {
    let az_rad = aer.azimuth_deg.to_radians();
    let el_rad = aer.elevation_deg.to_radians();
    let horiz = aer.range_m * el_rad.cos();
    let zenith = aer.range_m * el_rad.sin();
    let north = horiz * az_rad.cos();
    let east = horiz * az_rad.sin();
    SezCoord {
        south_m: -north,
        east_m: east,
        zenith_m: zenith,
    }
}
