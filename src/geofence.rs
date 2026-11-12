//! Tactical Geofencing Engine & Polygon Breach Detection.

/// Breach classification status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeofenceState {
    Inside,
    Outside,
    Entered,
    Exited,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeofenceBreach {
    pub state: GeofenceState,
    pub distance_to_boundary_m: f64,
}

pub const MAX_GEOFENCE_VERTICES: usize = 32;

/// 3D Tactical Geofence with polygon boundary and altitude envelope.
#[derive(Debug, Clone, Copy)]
pub struct Geofence {
    pub vertices: [[f64; 2]; MAX_GEOFENCE_VERTICES],
    pub vertex_count: usize,
    pub min_alt_msl: f64,
    pub max_alt_msl: f64,
}
