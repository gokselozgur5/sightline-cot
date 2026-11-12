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

