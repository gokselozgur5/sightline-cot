//! Rhumb line (loxodrome) navigation solver and waypoint interpolator.

/// Default mean Earth radius in meters (WGS84 spherical approximation).
pub use crate::geo_greatcircle::EARTH_RADIUS_M;

/// WGS84 mean spherical Earth radius alias in meters.
pub const WGS84_EARTH_RADIUS: f64 = EARTH_RADIUS_M;

/// Maximum number of samples allowed along an interpolated path.
pub const MAX_SAMPLE_COUNT: usize = 100_000;

/// Error types encountered during rhumb line navigation and interpolation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RhumbLineError {
    /// Latitude value outside [-90.0, 90.0] or non-finite.
    InvalidLatitude,
    /// Longitude value outside [-180.0, 180.0] or non-finite.
    InvalidLongitude,
    /// Fewer than two waypoints provided for path.
    InsufficientWaypoints,
    /// Step distance or fraction is invalid, non-positive, or NaN.
    InvalidParameter,
}

/// Geographic waypoint in decimal degrees.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RhumbWaypoint {
    pub lat: f64,
    pub lon: f64,
}

impl RhumbWaypoint {
    /// Create a new waypoint with latitude and longitude.
    pub const fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }

    /// Validate latitude in [-90, 90] and longitude in [-180, 180].
    pub fn validate(&self) -> Result<(), RhumbLineError> {
        if !self.lat.is_finite() || !(-90.0..=90.0).contains(&self.lat) {
            return Err(RhumbLineError::InvalidLatitude);
        }
        if !self.lon.is_finite() || !(-180.0..=180.0).contains(&self.lon) {
            return Err(RhumbLineError::InvalidLongitude);
        }
        Ok(())
    }
}

use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

const MAX_MERCATOR_LAT: f64 = FRAC_PI_2 - 1e-15;

fn mercator_project(lat_rad: f64) -> f64 {
    let clamped = lat_rad.clamp(-MAX_MERCATOR_LAT, MAX_MERCATOR_LAT);
    (FRAC_PI_4 + clamped / 2.0).tan().ln()
}

fn unwrap_lon_diff_rad(lon1_deg: f64, lon2_deg: f64) -> f64 {
    let diff = (lon2_deg - lon1_deg + 540.0).rem_euclid(360.0) - 180.0;
    diff.to_radians()
}

/// Computes the rhumb line distance between two `[lat, lon]` points on a custom sphere radius.
pub fn rhumb_distance_radius(p1: [f64; 2], p2: [f64; 2], radius: f64) -> f64 {
    let (lat1, lat2) = (p1[0].to_radians(), p2[0].to_radians());
    let dlat = lat2 - lat1;
    let dlon = unwrap_lon_diff_rad(p1[1], p2[1]);
    let dpsi = mercator_project(lat2) - mercator_project(lat1);
    let q = if dpsi.abs() < 1e-12 {
        lat1.cos()
    } else {
        dlat / dpsi
    };
    radius * (dlat * dlat + q * q * dlon * dlon).sqrt()
}

/// Computes the rhumb line distance between two `[lat, lon]` points on WGS84 mean radius.
pub fn rhumb_distance(p1: [f64; 2], p2: [f64; 2]) -> f64 {
    rhumb_distance_radius(p1, p2, EARTH_RADIUS_M)
}

/// Computes the constant rhumb line bearing in degrees in `[0, 360)`.
pub fn rhumb_bearing(p1: [f64; 2], p2: [f64; 2]) -> f64 {
    let (lat1, lat2) = (p1[0].to_radians(), p2[0].to_radians());
    let dlon = unwrap_lon_diff_rad(p1[1], p2[1]);
    let dpsi = mercator_project(lat2) - mercator_project(lat1);
    dlon.atan2(dpsi).to_degrees().rem_euclid(360.0)
}

/// Computes the destination `[lat, lon]` along a rhumb line on a custom sphere radius.
pub fn rhumb_destination_radius(
    start: [f64; 2],
    bearing_deg: f64,
    distance_m: f64,
    radius: f64,
) -> [f64; 2] {
    let (lat1, lon1) = (start[0].to_radians(), start[1].to_radians());
    let theta = bearing_deg.to_radians();
    let delta = distance_m / radius;
    let lat2 = (lat1 + delta * theta.cos()).clamp(-FRAC_PI_2, FRAC_PI_2);
    let dpsi = mercator_project(lat2) - mercator_project(lat1);
    let q = if dpsi.abs() < 1e-12 {
        lat1.cos()
    } else {
        (lat2 - lat1) / dpsi
    };
    let dlon = if q.abs() < 1e-12 {
        0.0
    } else {
        (delta * theta.sin()) / q
    };
    let lon2 = ((lon1 + dlon).to_degrees() + 180.0).rem_euclid(360.0) - 180.0;
    [lat2.to_degrees(), lon2]
}

/// Computes the destination `[lat, lon]` along a rhumb line on WGS84 mean radius.
pub fn rhumb_destination(start: [f64; 2], bearing_deg: f64, distance_m: f64) -> [f64; 2] {
    rhumb_destination_radius(start, bearing_deg, distance_m, EARTH_RADIUS_M)
}

/// Computes rhumb line distance (meters) and constant bearing (degrees).
pub fn rhumb_distance_and_bearing(
    p1: &RhumbWaypoint,
    p2: &RhumbWaypoint,
    radius: f64,
) -> Result<(f64, f64), RhumbLineError> {
    p1.validate()?;
    p2.validate()?;
    let (a, b) = ([p1.lat, p1.lon], [p2.lat, p2.lon]);
    Ok((rhumb_distance_radius(a, b, radius), rhumb_bearing(a, b)))
}

/// Computes great-circle distance between two waypoints using haversine formula.
pub fn great_circle_distance(
    p1: &RhumbWaypoint,
    p2: &RhumbWaypoint,
    radius: f64,
) -> Result<f64, RhumbLineError> {
    p1.validate()?;
    p2.validate()?;
    let d = crate::geo_greatcircle::haversine_distance(p1.lat, p1.lon, p2.lat, p2.lon);
    Ok(d * (radius / crate::geo_greatcircle::EARTH_RADIUS_M))
}

/// Comparison between rhumb line and great-circle navigation along a segment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RhumbSegmentComparison {
    pub rhumb_distance_m: f64,
    pub great_circle_distance_m: f64,
    pub rhumb_bearing_deg: f64,
    pub distance_difference_m: f64,
}

impl RhumbSegmentComparison {
    /// Compare rhumb line and great-circle trajectories between two waypoints.
    pub fn compute(p1: &RhumbWaypoint, p2: &RhumbWaypoint) -> Result<Self, RhumbLineError> {
        let (rhumb_dist, bearing) = rhumb_distance_and_bearing(p1, p2, EARTH_RADIUS_M)?;
        let gc_dist = great_circle_distance(p1, p2, EARTH_RADIUS_M)?;
        Ok(Self {
            rhumb_distance_m: rhumb_dist,
            great_circle_distance_m: gc_dist,
            rhumb_bearing_deg: bearing,
            distance_difference_m: (rhumb_dist - gc_dist).max(0.0),
        })
    }
}

fn rhumb_interp_raw(p1: [f64; 2], p2: [f64; 2], f: f64) -> [f64; 2] {
    let (lat1, lat2) = (p1[0].to_radians(), p2[0].to_radians());
    let dlat = lat2 - lat1;
    let dlon = unwrap_lon_diff_rad(p1[1], p2[1]);
    let lat_f = lat1 + f * dlat;
    let dpsi = mercator_project(lat2) - mercator_project(lat1);
    let dlon_f = if dpsi.abs() < 1e-12 {
        f * dlon
    } else {
        (mercator_project(lat_f) - mercator_project(lat1)) * (dlon / dpsi)
    };
    let lon_f = (p1[1] + dlon_f.to_degrees() + 180.0).rem_euclid(360.0) - 180.0;
    [lat_f.to_degrees(), lon_f]
}

/// Computes an intermediate `[lat, lon]` along a rhumb line clamped to `[0, 1]`.
pub fn rhumb_intermediate(p1: [f64; 2], p2: [f64; 2], f: f64) -> [f64; 2] {
    if f <= 0.0 {
        return p1;
    }
    if f >= 1.0 {
        return p2;
    }
    rhumb_interp_raw(p1, p2, f)
}

/// Computes an intermediate waypoint along a rhumb line at fraction f in [0, 1].
pub fn rhumb_intermediate_fraction(
    p1: &RhumbWaypoint,
    p2: &RhumbWaypoint,
    f: f64,
) -> Result<RhumbWaypoint, RhumbLineError> {
    p1.validate()?;
    p2.validate()?;
    if !f.is_finite() || !(0.0..=1.0).contains(&f) {
        return Err(RhumbLineError::InvalidParameter);
    }
    let [lat, lon] = rhumb_interp_raw([p1.lat, p1.lon], [p2.lat, p2.lon], f);
    Ok(RhumbWaypoint::new(lat, lon))
}

/// Interpolates intermediate waypoints along a rhumb line by count.
pub fn interpolate_by_count(
    p1: &RhumbWaypoint,
    p2: &RhumbWaypoint,
    count: usize,
) -> Result<Vec<RhumbWaypoint>, RhumbLineError> {
    p1.validate()?;
    p2.validate()?;
    if count == 0 || count > MAX_SAMPLE_COUNT {
        return Err(RhumbLineError::InvalidParameter);
    }
    let mut waypoints = Vec::with_capacity(count + 2);
    waypoints.push(*p1);
    for i in 1..=count {
        let f = (i as f64) / ((count + 1) as f64);
        waypoints.push(rhumb_intermediate_fraction(p1, p2, f)?);
    }
    waypoints.push(*p2);
    Ok(waypoints)
}

/// Interpolates intermediate waypoints along a rhumb line by max step distance in meters.
pub fn interpolate_by_step(
    p1: &RhumbWaypoint,
    p2: &RhumbWaypoint,
    step_m: f64,
) -> Result<Vec<RhumbWaypoint>, RhumbLineError> {
    if !step_m.is_finite() || step_m <= 0.0 {
        return Err(RhumbLineError::InvalidParameter);
    }
    let (total_dist, _) = rhumb_distance_and_bearing(p1, p2, EARTH_RADIUS_M)?;
    if total_dist <= step_m {
        return Ok(vec![*p1, *p2]);
    }
    let steps_f = (total_dist / step_m).ceil();
    if !steps_f.is_finite() || steps_f > MAX_SAMPLE_COUNT as f64 {
        return Err(RhumbLineError::InvalidParameter);
    }
    let steps = steps_f as usize;
    let mut waypoints = Vec::with_capacity(steps + 1);
    waypoints.push(*p1);
    for i in 1..steps {
        let f = (i as f64) / (steps as f64);
        waypoints.push(rhumb_intermediate_fraction(p1, p2, f)?);
    }
    waypoints.push(*p2);
    Ok(waypoints)
}

/// Multi-waypoint rhumb line flight or maritime navigation path.
#[derive(Debug, Clone, PartialEq)]
pub struct RhumbPath {
    waypoints: Vec<RhumbWaypoint>,
    segment_rhumb_distances: Vec<f64>,
    segment_gc_distances: Vec<f64>,
    total_rhumb_distance: f64,
    total_gc_distance: f64,
}

impl RhumbPath {
    /// Return the list of waypoints in the path.
    pub fn waypoints(&self) -> &[RhumbWaypoint] {
        &self.waypoints
    }

    /// Return total rhumb distance in meters.
    pub fn total_rhumb_distance(&self) -> f64 {
        self.total_rhumb_distance
    }

    /// Return total great circle distance in meters.
    pub fn total_gc_distance(&self) -> f64 {
        self.total_gc_distance
    }
}

impl RhumbPath {
    /// Constructs a multi-segment rhumb path from a sequence of waypoints.
    pub fn from_waypoints(waypoints: Vec<RhumbWaypoint>) -> Result<Self, RhumbLineError> {
        if waypoints.len() < 2 {
            return Err(RhumbLineError::InsufficientWaypoints);
        }
        for wp in &waypoints {
            wp.validate()?;
        }
        let (mut seg_rhumb, mut seg_gc) = (Vec::new(), Vec::new());
        let (mut total_rhumb, mut total_gc) = (0.0, 0.0);
        for w in waypoints.windows(2) {
            let (rd, _) = rhumb_distance_and_bearing(&w[0], &w[1], EARTH_RADIUS_M)?;
            let gd = great_circle_distance(&w[0], &w[1], EARTH_RADIUS_M)?;
            seg_rhumb.push(rd);
            seg_gc.push(gd);
            total_rhumb += rd;
            total_gc += gd;
        }
        Ok(Self {
            waypoints,
            segment_rhumb_distances: seg_rhumb,
            segment_gc_distances: seg_gc,
            total_rhumb_distance: total_rhumb,
            total_gc_distance: total_gc,
        })
    }
}

impl RhumbPath {
    /// Return the count of segments in the path.
    pub fn segment_count(&self) -> usize {
        self.waypoints.len() - 1
    }

    /// Resample the entire path with a maximum distance step between points.
    pub fn resample_by_step(&self, step_m: f64) -> Result<Vec<RhumbWaypoint>, RhumbLineError> {
        let mut full_path = Vec::new();
        for i in 0..self.segment_count() {
            let seg_points =
                interpolate_by_step(&self.waypoints[i], &self.waypoints[i + 1], step_m)?;
            if i > 0 {
                full_path.extend_from_slice(&seg_points[1..]);
            } else {
                full_path.extend(seg_points);
            }
        }
        Ok(full_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equator_rhumb_distance_and_bearing() {
        let p1 = RhumbWaypoint::new(0.0, 10.0);
        let p2 = RhumbWaypoint::new(0.0, 20.0);
        let (dist, bearing) = rhumb_distance_and_bearing(&p1, &p2, EARTH_RADIUS_M).unwrap();
        assert!((bearing - 90.0).abs() < 1e-6);
        let expected_dist = EARTH_RADIUS_M * (10.0_f64.to_radians());
        assert!((dist - expected_dist).abs() < 1.0);
    }

    #[test]
    fn test_meridian_rhumb_distance_and_bearing() {
        let p1 = RhumbWaypoint::new(10.0, 50.0);
        let p2 = RhumbWaypoint::new(20.0, 50.0);
        let (dist, bearing) = rhumb_distance_and_bearing(&p1, &p2, EARTH_RADIUS_M).unwrap();
        assert!((bearing - 0.0).abs() < 1e-6);
        let expected_dist = EARTH_RADIUS_M * (10.0_f64.to_radians());
        assert!((dist - expected_dist).abs() < 1.0);
    }

    #[test]
    fn test_rhumb_vs_great_circle_comparison() {
        let ny = RhumbWaypoint::new(40.7128, -74.0060);
        let london = RhumbWaypoint::new(51.5074, -0.1278);
        let comp = RhumbSegmentComparison::compute(&ny, &london).unwrap();
        assert!(comp.rhumb_distance_m > comp.great_circle_distance_m);
        assert!(comp.distance_difference_m > 0.0);
        assert!(comp.rhumb_bearing_deg > 0.0 && comp.rhumb_bearing_deg < 90.0);
    }

    #[test]
    fn test_interpolate_by_count_and_step() {
        let p1 = RhumbWaypoint::new(10.0, 10.0);
        let p2 = RhumbWaypoint::new(10.0, 20.0);
        let count_pts = interpolate_by_count(&p1, &p2, 3).unwrap();
        assert_eq!(count_pts.len(), 5);
        assert!((count_pts[2].lon - 15.0).abs() < 1e-4);

        let step_pts = interpolate_by_step(&p1, &p2, 200_000.0).unwrap();
        assert!(step_pts.len() > 2);
    }

    #[test]
    fn test_rhumb_path_and_errors() {
        let wps = vec![
            RhumbWaypoint::new(0.0, 0.0),
            RhumbWaypoint::new(10.0, 10.0),
            RhumbWaypoint::new(20.0, 20.0),
        ];
        let path = RhumbPath::from_waypoints(wps).unwrap();
        assert_eq!(path.segment_count(), 2);
        assert!(path.total_rhumb_distance() > 0.0);

        let invalid_wp = RhumbWaypoint::new(95.0, 0.0);
        assert_eq!(invalid_wp.validate(), Err(RhumbLineError::InvalidLatitude));
    }
}
