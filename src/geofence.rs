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

impl Geofence {
    pub fn new(poly: &[[f64; 2]], min_alt: f64, max_alt: f64) -> Result<Self, &'static str> {
        if poly.len() < 3 || poly.len() > MAX_GEOFENCE_VERTICES {
            return Err("vertex count must be between 3 and 32");
        }
        let mut vertices = [[0.0; 2]; MAX_GEOFENCE_VERTICES];
        vertices[..poly.len()].copy_from_slice(poly);
        Ok(Self {
            vertices,
            vertex_count: poly.len(),
            min_alt_msl: min_alt,
            max_alt_msl: max_alt,
        })
    }
}

impl Geofence {
    pub fn is_altitude_valid(&self, alt_msl: f64) -> bool {
        alt_msl >= self.min_alt_msl && alt_msl <= self.max_alt_msl
    }
}

impl Geofence {
    pub fn is_point_inside_polygon(&self, lat: f64, lon: f64) -> bool {
        let mut inside = false;
        let n = self.vertex_count;
        let mut j = n - 1;
        for i in 0..n {
            let (xi, yi) = (self.vertices[i][1], self.vertices[i][0]);
            let (xj, yj) = (self.vertices[j][1], self.vertices[j][0]);
            let intersect =
                ((yi > lat) != (yj > lat)) && (lon < (xj - xi) * (lat - yi) / (yj - yi) + xi);
            if intersect {
                inside = !inside;
            }
            j = i;
        }
        inside
    }
}

impl Geofence {
    pub fn distance_to_boundary_m(&self, lat: f64, lon: f64) -> f64 {
        let mut min_dist_deg_sq = f64::MAX;
        let n = self.vertex_count;
        for i in 0..n {
            let dlat = self.vertices[i][0] - lat;
            let dlon = (self.vertices[i][1] - lon) * lat.to_radians().cos();
            let dist_sq = dlat * dlat + dlon * dlon;
            if dist_sq < min_dist_deg_sq {
                min_dist_deg_sq = dist_sq;
            }
        }
        min_dist_deg_sq.sqrt() * 111_319.5
    }
}

impl Geofence {
    fn classify_transition(is_inside: bool, prev: Option<bool>) -> GeofenceState {
        match (is_inside, prev) {
            (true, Some(false)) => GeofenceState::Entered,
            (false, Some(true)) => GeofenceState::Exited,
            (true, _) => GeofenceState::Inside,
            (false, _) => GeofenceState::Outside,
        }
    }
}

impl Geofence {
    pub fn evaluate_breach(
        &self,
        lat: f64,
        lon: f64,
        alt: f64,
        prev: Option<bool>,
    ) -> GeofenceBreach {
        let inside = self.is_point_inside_polygon(lat, lon) && self.is_altitude_valid(alt);
        let dist = self.distance_to_boundary_m(lat, lon);
        let state = Self::classify_transition(inside, prev);
        GeofenceBreach {
            state,
            distance_to_boundary_m: dist,
        }
    }
}
