// Tactical geofencing engine and polygon breach detection.

pub const MAX_GEOFENCE_VERTICES: usize = 32;

/// Geofence breach event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeofenceStatus {
    Inside,
    Outside,
    Entered,
    Exited,
}

/// Breach report with closest distance to perimeter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeofenceBreach {
    pub status: GeofenceStatus,
    pub distance_to_boundary_m: f64,
}

/// 2D geographic vertex (lat/lon in degrees).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeofencePoint {
    pub lat_deg: f64,
    pub lon_deg: f64,
}

/// 3D polygonal tactical geofence with altitude limits.
#[derive(Debug, Clone, PartialEq)]
pub struct Geofence {
    pub vertices: [GeofencePoint; MAX_GEOFENCE_VERTICES],
    pub vertex_count: usize,
    pub min_alt_m: f64,
    pub max_alt_m: f64,
}

impl Geofence {
    /// Construct a 3D tactical geofence from vertices and altitude bounds.
    pub fn new(
        poly: &[GeofencePoint],
        min_alt_m: f64,
        max_alt_m: f64,
    ) -> Result<Self, &'static str> {
        if poly.len() < 3 || poly.len() > MAX_GEOFENCE_VERTICES {
            return Err("vertex count must be between 3 and 32");
        }
        if min_alt_m > max_alt_m || !min_alt_m.is_finite() || !max_alt_m.is_finite() {
            return Err("invalid altitude bounds");
        }
        let mut vertices = [GeofencePoint {
            lat_deg: 0.0,
            lon_deg: 0.0,
        }; MAX_GEOFENCE_VERTICES];
        for (i, p) in poly.iter().enumerate() {
            if !p.lat_deg.is_finite() || !p.lon_deg.is_finite() {
                return Err("non-finite coordinate");
            }
            vertices[i] = *p;
        }
        Ok(Self {
            vertices,
            vertex_count: poly.len(),
