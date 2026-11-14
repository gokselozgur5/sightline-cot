//! Line-of-sight ray ground and planar surface intersection.

/// Errors occurring during line-of-sight ground intersection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LosGroundError {
    /// Ray direction is horizontal, pointing upwards, or away from ground.
    RayAboveOrParallelGround,
    /// Ray direction vector has zero or near-zero magnitude.
    DegenerateRayDirection,
    /// One or more coordinates or vector components are non-finite.
    NonFiniteInput,
}

impl core::fmt::Display for LosGroundError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RayAboveOrParallelGround => {
                write!(f, "ray is pointing above or parallel to ground")
            }
            Self::DegenerateRayDirection => write!(f, "ray direction vector is degenerate"),
            Self::NonFiniteInput => write!(f, "input coordinates are non-finite"),
        }
    }
}

impl std::error::Error for LosGroundError {}

/// 3D ray defined by origin point and direction vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LosRay {
    pub origin: [f64; 3],
    pub direction: [f64; 3],
}

impl LosRay {
    /// Create and validate a line-of-sight ray.
    pub fn new(origin: [f64; 3], direction: [f64; 3]) -> Result<Self, LosGroundError> {
        if !origin.iter().all(|x| x.is_finite()) || !direction.iter().all(|x| x.is_finite()) {
            return Err(LosGroundError::NonFiniteInput);
        }
        let mag_sq = direction[0] * direction[0]
            + direction[1] * direction[1]
            + direction[2] * direction[2];
        if mag_sq <= 1e-18 {
            return Err(LosGroundError::DegenerateRayDirection);
        }
        Ok(Self { origin, direction })
    }
}

/// Result of a successful line-of-sight ground intersection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LosGroundIntersection {
    /// 3D coordinates of intersection [east, north, up].
    pub point: [f64; 3],
    /// Slant range from origin to ground point in meters.
    pub slant_range_m: f64,
    /// Distance parameter t along the ray direction vector.
    pub t: f64,
}

fn check_ray_ground_t(ray: &LosRay, ground_up_m: f64) -> Result<f64, LosGroundError> {
    if !ground_up_m.is_finite() {
        return Err(LosGroundError::NonFiniteInput);
    }
    let dz = ray.direction[2];
    let delta_z = ground_up_m - ray.origin[2];
    if (dz >= -1e-9 && delta_z <= 0.0) || (dz <= 1e-9 && delta_z >= 0.0) {
        return Err(LosGroundError::RayAboveOrParallelGround);
    }
    let t = delta_z / dz;
    if t < 0.0 {
        return Err(LosGroundError::RayAboveOrParallelGround);
    }
    Ok(t)
}

/// Intersect line-of-sight ray with horizontal ground plane at `ground_up_m`.
pub fn intersect_ground_plane(
    ray: &LosRay,
    ground_up_m: f64,
) -> Result<LosGroundIntersection, LosGroundError> {
    let t = check_ray_ground_t(ray, ground_up_m)?;
    let point = [
        ray.origin[0] + t * ray.direction[0],
        ray.origin[1] + t * ray.direction[1],
        ground_up_m,
    ];
    let dx = point[0] - ray.origin[0];
    let dy = point[1] - ray.origin[1];
    let dz = point[2] - ray.origin[2];
    let slant_range_m = (dx * dx + dy * dy + dz * dz).sqrt();
    Ok(LosGroundIntersection { point, slant_range_m, t })
}

fn sloped_plane_t(
    ray: &LosRay,
    ref_pt: [f64; 3],
    ge: f64,
    gn: f64,
) -> Result<f64, LosGroundError> {
    let denom = ray.direction[2] - ge * ray.direction[0] - gn * ray.direction[1];
    if denom.abs() <= 1e-12 {
        return Err(LosGroundError::RayAboveOrParallelGround);
    }
    let num = ref_pt[2] - ray.origin[2] + ge * (ray.origin[0] - ref_pt[0])
        + gn * (ray.origin[1] - ref_pt[1]);
    let t = num / denom;
    if t < 0.0 {
        return Err(LosGroundError::RayAboveOrParallelGround);
    }
    Ok(t)
}

/// Intersect ray with a planar ground surface with slope gradients (ge, gn).
pub fn intersect_sloped_plane(
    ray: &LosRay,
    ref_point: [f64; 3],
    grad_e: f64,
    grad_n: f64,
) -> Result<LosGroundIntersection, LosGroundError> {
    if !ref_point.iter().all(|x| x.is_finite()) || !grad_e.is_finite() || !grad_n.is_finite() {
        return Err(LosGroundError::NonFiniteInput);
    }
    let t = sloped_plane_t(ray, ref_point, grad_e, grad_n)?;
    let point = [
        ray.origin[0] + t * ray.direction[0],
        ray.origin[1] + t * ray.direction[1],
        ray.origin[2] + t * ray.direction[2],
    ];
    let dx = point[0] - ray.origin[0];
    let dy = point[1] - ray.origin[1];
    let dz = point[2] - ray.origin[2];
    let slant_range_m = (dx * dx + dy * dy + dz * dz).sqrt();
    Ok(LosGroundIntersection { point, slant_range_m, t })
}

use crate::camera::CameraPose;

/// Back-project camera optical line of sight to ground plane.
pub fn camera_los_ground(
    pose: &CameraPose,
    ray_cam: [f64; 3],
    ground_up_m: f64,
) -> Result<LosGroundIntersection, LosGroundError> {
    let ray_enu = pose.transform_ray(ray_cam);
    let cam_origin = [0.0, 0.0, pose.cam_up_m];
    let los_ray = LosRay::new(cam_origin, ray_enu)?;
    intersect_ground_plane(&los_ray, ground_up_m)
}
