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
            min_alt_m,
            max_alt_m,
        })
    }

    fn is_left(p0: GeofencePoint, p1: GeofencePoint, p2: GeofencePoint) -> f64 {
        (p1.lon_deg - p0.lon_deg) * (p2.lat_deg - p0.lat_deg)
            - (p2.lon_deg - p0.lon_deg) * (p1.lat_deg - p0.lat_deg)
    }

    /// Point-in-polygon test using winding number algorithm.
    pub fn contains_horizontal(&self, pt: GeofencePoint) -> bool {
        let mut wn: i32 = 0;
        let n = self.vertex_count;
        for i in 0..n {
            let v_curr = self.vertices[i];
            let v_next = self.vertices[(i + 1) % n];
            if v_curr.lat_deg <= pt.lat_deg {
                if v_next.lat_deg > pt.lat_deg && Self::is_left(v_curr, v_next, pt) > 0.0 {
                    wn += 1;
                }
            } else if v_next.lat_deg <= pt.lat_deg && Self::is_left(v_curr, v_next, pt) < 0.0 {
                wn -= 1;
            }
        }
        wn != 0
    }

    /// 3D containment check including altitude MSL window.
    pub fn contains_3d(&self, lat_deg: f64, lon_deg: f64, alt_m: f64) -> bool {
        if alt_m < self.min_alt_m || alt_m > self.max_alt_m {
            return false;
        }
        self.contains_horizontal(GeofencePoint { lat_deg, lon_deg })
    }

    fn dist_to_segment_m(p: GeofencePoint, a: GeofencePoint, b: GeofencePoint) -> f64 {
        let mean_lat_rad = (a.lat_deg + b.lat_deg) * 0.5 * std::f64::consts::PI / 180.0;
        let k_lat = 111132.95;
        let k_lon = 111412.84 * mean_lat_rad.cos();
        let ax = (a.lon_deg - p.lon_deg) * k_lon;
        let ay = (a.lat_deg - p.lat_deg) * k_lat;
        let bx = (b.lon_deg - p.lon_deg) * k_lon;
        let by = (b.lat_deg - p.lat_deg) * k_lat;
        let dx = bx - ax;
        let dy = by - ay;
        let seg_len_sq = dx * dx + dy * dy;
        if seg_len_sq < 1e-6 {
            return (ax * ax + ay * ay).sqrt();
        }
        let t = ((-ax * dx - ay * dy) / seg_len_sq).clamp(0.0, 1.0);
        let px = ax + t * dx;
