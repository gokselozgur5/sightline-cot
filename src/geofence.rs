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
    fn polygon_area2(poly: &[[f64; 2]]) -> f64 {
        let n = poly.len();
        let mut x = poly[0][1];
        (0..n)
            .map(|i| {
                let (a, b) = (poly[i], poly[(i + 1) % n]);
                let xb = x + Self::lon_step(a[1], b[1]);
                let term = x * b[0] - xb * a[0];
                x = xb;
                term
            })
            .sum()
    }
}

impl Geofence {
    /// Builds a geofence from `[lat, lon]` vertices.
    ///
    /// Consecutive vertices are joined the short way round in longitude, so every edge must
    /// span less than 180 degrees. A ring that winds once around a pole encloses the pole
    /// in the hemisphere of its mean vertex latitude.
    pub fn new(poly: &[[f64; 2]], min_alt: f64, max_alt: f64) -> Result<Self, &'static str> {
        if poly.len() < 3 || poly.len() > MAX_GEOFENCE_VERTICES {
            return Err("vertex count must be between 3 and 32");
        }
        if !(min_alt.is_finite() && max_alt.is_finite()) || min_alt > max_alt {
            return Err("altitude envelope must be finite with min <= max");
        }
        if poly.iter().flatten().any(|c| !c.is_finite()) {
            return Err("polygon coordinates must be finite");
        }
        if Self::pole_winding(poly) == 0 && Self::polygon_area2(poly).abs() < 1e-12 {
            return Err("polygon must have non-zero area");
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
    /// Unwraps `lon` into the 360 degree window centered on `reference`.
    fn unwrap_lon(lon: f64, reference: f64) -> f64 {
        reference + ((lon - reference + 540.0).rem_euclid(360.0) - 180.0)
    }

    /// Signed longitude step from `a` to `b`, taking the short way round.
    fn lon_step(a: f64, b: f64) -> f64 {
        Self::unwrap_lon(b, a) - a
    }

    /// Net longitude swept by the ring in turns: 0 for an ordinary ring, +1 or -1 around a pole.
    fn pole_winding(poly: &[[f64; 2]]) -> i32 {
        let n = poly.len();
        let sweep: f64 = (0..n)
            .map(|i| Self::lon_step(poly[i][1], poly[(i + 1) % n][1]))
            .sum();
        (sweep / 360.0).round() as i32
    }
}

impl Geofence {
    /// True when the meridian through `lon` crosses edge `a`-`b` north of `lat`.
    fn crosses_north(a: [f64; 2], b: [f64; 2], lat: f64, lon: f64) -> bool {
        let dx = Self::lon_step(a[1], b[1]);
        let lo = dx.min(0.0);
        let s = (lon - a[1] - lo).rem_euclid(360.0);
        if dx == 0.0 || s >= dx.abs() {
            return false;
        }
        let t = (s + lo) / dx;
        a[0] + t * (b[0] - a[0]) > lat
    }
}

impl Geofence {
    pub fn is_point_inside_polygon(&self, lat: f64, lon: f64) -> bool {
        let poly = &self.vertices[..self.vertex_count];
        let n = poly.len();
        let crossings = (0..n)
            .filter(|&i| Self::crosses_north(poly[i], poly[(i + 1) % n], lat, lon))
            .count();
        let north_cap = poly.iter().map(|v| v[0]).sum::<f64>() >= 0.0;
        (crossings % 2 == 1) != (Self::pole_winding(poly) != 0 && north_cap)
    }
}

impl Geofence {
    pub fn distance_to_boundary_m(&self, lat: f64, lon: f64) -> f64 {
        let mut min_dist_deg_sq = f64::MAX;
        let n = self.vertex_count;
        let k = lat.to_radians().cos();
        for i in 0..n {
            let a = self.vertices[i];
            let b = self.vertices[(i + 1) % n];
            let dist_sq = Self::segment_dist_sq((lon * k, lat), (a[1] * k, a[0]), (b[1] * k, b[0]));
            if dist_sq < min_dist_deg_sq {
                min_dist_deg_sq = dist_sq;
            }
        }
        min_dist_deg_sq.sqrt() * 111_319.5
    }
}

impl Geofence {
    fn segment_dist_sq(p: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let len_sq = dx * dx + dy * dy;
        let t = if len_sq == 0.0 {
            0.0
        } else {
            (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / len_sq).clamp(0.0, 1.0)
        };
        let (cx, cy) = (a.0 + t * dx, a.1 + t * dy);
        (p.0 - cx).powi(2) + (p.1 - cy).powi(2)
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

impl Geofence {
    /// Like `evaluate_breach`, but keeps the previous side while within `margin_m` of the boundary.
    pub fn evaluate_breach_hysteresis(
        &self,
        lat: f64,
        lon: f64,
        alt: f64,
        prev: Option<bool>,
        margin_m: f64,
    ) -> GeofenceBreach {
        let raw = self.evaluate_breach(lat, lon, alt, prev);
        let now_inside = matches!(raw.state, GeofenceState::Inside | GeofenceState::Entered);
        let flipped = matches!(prev, Some(p) if p != now_inside);
        if !flipped || raw.distance_to_boundary_m >= margin_m {
            return raw;
        }
        let state = if now_inside {
            GeofenceState::Outside
        } else {
            GeofenceState::Inside
        };
        GeofenceBreach { state, ..raw }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geofence_creation_and_containment() {
        let square = [
            [37.0, -122.0],
            [37.0, -121.0],
            [38.0, -121.0],
            [38.0, -122.0],
        ];
        let gf = Geofence::new(&square, 100.0, 500.0).unwrap();
        assert!(gf.is_point_inside_polygon(37.5, -121.5));
        assert!(!gf.is_point_inside_polygon(36.5, -121.5));
        assert!(gf.is_altitude_valid(250.0));
        assert!(!gf.is_altitude_valid(50.0));
    }
}

#[test]
fn test_geofence_breach_event_transitions() {
    let poly = [
        [37.0, -122.0],
        [37.0, -121.0],
        [38.0, -121.0],
        [38.0, -122.0],
    ];
    let gf = Geofence::new(&poly, 0.0, 1000.0).unwrap();

    let b1 = gf.evaluate_breach(37.5, -121.5, 500.0, None);
    assert_eq!(b1.state, GeofenceState::Inside);

    let b2 = gf.evaluate_breach(37.5, -121.5, 500.0, Some(false));
    assert_eq!(b2.state, GeofenceState::Entered);

    let b3 = gf.evaluate_breach(36.0, -121.5, 500.0, Some(true));
    assert_eq!(b3.state, GeofenceState::Exited);

    let b4 = gf.evaluate_breach(36.0, -121.5, 500.0, Some(false));
    assert_eq!(b4.state, GeofenceState::Outside);
}
