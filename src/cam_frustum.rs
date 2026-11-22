//! Camera frustum bounding volume generation in local ENU and geodetic frames.

use crate::camera::{enu_to_geodetic, CameraPose, Intrinsics};
use crate::geo_bbox::GeoBbox;
pub use crate::sensor_frustum::FrustumError;

/// Axis-aligned bounding box in 3D ENU frame [min_east, min_north, min_up] to [max_east, max_north, max_up].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnuBbox {
    pub min_e: f64,
    pub min_n: f64,
    pub min_u: f64,
    pub max_e: f64,
    pub max_n: f64,
    pub max_u: f64,
}

impl EnuBbox {
    /// Create an EnuBbox enclosing a slice of 3D points.
    pub fn from_points(points: &[[f64; 3]]) -> Option<Self> {
        let first = points.first()?;
        let (mut min_e, mut max_e) = (first[0], first[0]);
        let (mut min_n, mut max_n) = (first[1], first[1]);
        let (mut min_u, mut max_u) = (first[2], first[2]);
        for p in &points[1..] {
            min_e = min_e.min(p[0]);
            max_e = max_e.max(p[0]);
            min_n = min_n.min(p[1]);
            max_n = max_n.max(p[1]);
            min_u = min_u.min(p[2]);
            max_u = max_u.max(p[2]);
        }
        Some(Self {
            min_e,
            min_n,
            min_u,
            max_e,
            max_n,
            max_u,
        })
    }
}

impl EnuBbox {
    /// Check whether a 3D ENU point lies within or on the bounding box.
    pub fn contains(&self, p: [f64; 3]) -> bool {
        p[0] >= self.min_e
            && p[0] <= self.max_e
            && p[1] >= self.min_n
            && p[1] <= self.max_n
            && p[2] >= self.min_u
            && p[2] <= self.max_u
    }

    /// Center of the 3D bounding box.
    pub fn center(&self) -> [f64; 3] {
        [
            (self.min_e + self.max_e) * 0.5,
            (self.min_n + self.max_n) * 0.5,
            (self.min_u + self.max_u) * 0.5,
        ]
    }
}

/// Camera frustum bounding volume in local ENU and geodetic frames.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraFrustum {
    pub corners_cam: [[f64; 3]; 8],
    pub corners_enu: [[f64; 3]; 8],
    pub enu_bbox: EnuBbox,
    pub geo_bbox: GeoBbox,
}

fn check_inputs(w: f64, h: f64, n: f64, f: f64, lat: f64, lon: f64, hae: f64) -> bool {
    w.is_finite()
        && w > 0.0
        && h.is_finite()
        && h > 0.0
        && n.is_finite()
        && n > 0.0
        && f.is_finite()
        && f > n
        && lat.is_finite()
        && (-90.0..=90.0).contains(&lat)
        && lon.is_finite()
        && hae.is_finite()
}

fn compute_cam_corners(
    cam: &Intrinsics,
    w: f64,
    h: f64,
    near_m: f64,
    far_m: f64,
) -> Result<[[f64; 3]; 8], FrustumError> {
    let px = [[0.0, 0.0], [w, 0.0], [w, h], [0.0, h]];
    let mut corners = [[0.0; 3]; 8];
    for (i, p) in px.iter().enumerate() {
        let ray = cam
            .pixel_to_ray(p[0], p[1])
            .map_err(|_| FrustumError::DegenerateFrustum)?;
        if ray[2] <= 0.0 {
            return Err(FrustumError::DegenerateFrustum);
        }
        let (s_near, s_far) = (near_m / ray[2], far_m / ray[2]);
        corners[i] = [ray[0] * s_near, ray[1] * s_near, near_m];
        corners[i + 4] = [ray[0] * s_far, ray[1] * s_far, far_m];
    }
    Ok(corners)
}

fn transform_frustum_corners(
    pose: &CameraPose,
    c_cam: &[[f64; 3]; 8],
    r_lat: f64,
    r_lon: f64,
    r_hae: f64,
) -> ([[f64; 3]; 8], [[f64; 2]; 8]) {
    let mut enu = [[0.0; 3]; 8];
    let mut geo = [[0.0; 2]; 8];
    for (i, c) in c_cam.iter().enumerate() {
        let p = pose.transform_ray(*c);
        enu[i] = [p[0], p[1], p[2] + pose.cam_up_m];
        let (lat, lon, _) = enu_to_geodetic(enu[i], r_lat, r_lon, r_hae);
        geo[i] = [lat, lon];
    }
    (enu, geo)
}

impl CameraFrustum {
    /// Compute frustum bounding volume in local ENU and geodetic frames.
    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        cam: &Intrinsics,
        pose: &CameraPose,
        w: f64,
        h: f64,
        near_m: f64,
        far_m: f64,
        ref_lat: f64,
        ref_lon: f64,
        ref_hae: f64,
    ) -> Result<Self, FrustumError> {
        if !check_inputs(w, h, near_m, far_m, ref_lat, ref_lon, ref_hae) {
            return Err(FrustumError::NonFiniteParameter("camera frustum"));
        }
        let corners_cam = compute_cam_corners(cam, w, h, near_m, far_m)?;
        let (corners_enu, geo_pts) =
            transform_frustum_corners(pose, &corners_cam, ref_lat, ref_lon, ref_hae);
        let enu_bbox = EnuBbox::from_points(&corners_enu).ok_or(FrustumError::DegenerateFrustum)?;
        let geo_bbox = GeoBbox::from_points(&geo_pts).ok_or(FrustumError::DegenerateFrustum)?;
        Ok(Self {
            corners_cam,
            corners_enu,
            enu_bbox,
            geo_bbox,
        })
    }
}
