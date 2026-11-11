//! Monocular camera models, SO(3) attitude, and ground ranging.

/// Errors occurring during camera back-projection and ground ranging.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraError {
    /// Ray does not intersect ground plane (horizontal or pointing upward).
    RayAboveHorizon,
    /// Numerical solver failed to invert fisheye distortion.
    DistortionSolverFailed,
    /// Invalid input parameters such as non-positive focal lengths.
    InvalidParameters,
}

impl std::fmt::Display for CameraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RayAboveHorizon => write!(f, "ray is above or parallel to horizon"),
            Self::DistortionSolverFailed => write!(f, "fisheye distortion solver failed"),
            Self::InvalidParameters => write!(f, "invalid camera or pose parameters"),
        }
    }
}

impl std::error::Error for CameraError {}

/// Camera intrinsic calibration model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Intrinsics {
    /// Standard rectilinear pinhole camera.
    Pinhole { fx: f64, fy: f64, cx: f64, cy: f64 },
    /// Equidistant fisheye model with radial polynomial distortion.
    EquidistantFisheye {
        fx: f64,
        fy: f64,
        cx: f64,
        cy: f64,
        k1: f64,
        k2: f64,
    },
}

impl Intrinsics {
    fn invert_equidistant(r_d: f64, k1: f64, k2: f64) -> Result<f64, CameraError> {
        let mut theta = r_d;
        for _ in 0..15 {
            let t2 = theta * theta;
            let f = theta * (1.0 + k1 * t2 + k2 * t2 * t2) - r_d;
            let df = 1.0 + 3.0 * k1 * t2 + 5.0 * k2 * t2 * t2;
            if df.abs() < 1e-12 {
                return Err(CameraError::DistortionSolverFailed);
            }
            let step = f / df;
            theta -= step;
            if step.abs() < 1e-12 {
                return Ok(theta);
            }
        }
        Err(CameraError::DistortionSolverFailed)
    }
}

impl Intrinsics {
    fn unproject_pinhole(
        u: f64,
        v: f64,
        fx: f64,
        fy: f64,
        cx: f64,
        cy: f64,
    ) -> Result<[f64; 3], CameraError> {
        if fx <= 0.0 || fy <= 0.0 {
            return Err(CameraError::InvalidParameters);
        }
        let x = (u - cx) / fx;
        let y = (v - cy) / fy;
        let norm = (x * x + y * y + 1.0).sqrt();
        Ok([x / norm, y / norm, 1.0 / norm])
    }
}

impl Intrinsics {
    fn unproject_fisheye(
        u: f64,
        v: f64,
        fx: f64,
        fy: f64,
        cx: f64,
        cy: f64,
        k1: f64,
        k2: f64,
    ) -> Result<[f64; 3], CameraError> {
        if fx <= 0.0 || fy <= 0.0 {
            return Err(CameraError::InvalidParameters);
        }
        let x_d = (u - cx) / fx;
        let y_d = (v - cy) / fy;
        let r_d = (x_d * x_d + y_d * y_d).sqrt();
        if r_d < 1e-12 {
            return Ok([0.0, 0.0, 1.0]);
        }
        let theta = Self::invert_equidistant(r_d, k1, k2)?;
        let scale = theta.sin() / r_d;
        Ok([x_d * scale, y_d * scale, theta.cos()])
    }
}

impl Intrinsics {
    /// Back-project a pixel coordinate (u, v) into a unit camera-frame ray.
    pub fn pixel_to_ray(&self, u: f64, v: f64) -> Result<[f64; 3], CameraError> {
        match *self {
            Self::Pinhole { fx, fy, cx, cy } => Self::unproject_pinhole(u, v, fx, fy, cx, cy),
            Self::EquidistantFisheye {
                fx,
                fy,
                cx,
                cy,
                k1,
                k2,
            } => Self::unproject_fisheye(u, v, fx, fy, cx, cy, k1, k2),
        }
    }
}

/// Camera attitude angles in radians (Tait-Bryan Z-X-Y / ENU frame).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraPose {
    pub roll_rad: f64,
    pub pitch_rad: f64,
    pub yaw_rad: f64,
    pub cam_up_m: f64,
}

fn mat3_mult(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut out = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                out[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    out
}

impl CameraPose {
    /// Compute 3x3 rotation matrix mapping camera frame to ENU frame.
    /// Camera frame: X right, Y down, Z optical axis forward.
    /// At roll=0, pitch=0, yaw=0: camera points nadir (Z -> Down [0, 0, -1]).
    pub fn rotation_matrix(&self) -> [[f64; 3]; 3] {
        let (sr, cr) = (self.roll_rad.sin(), self.roll_rad.cos());
        let (sp, cp) = (self.pitch_rad.sin(), self.pitch_rad.cos());
        let (sy, cy) = (self.yaw_rad.sin(), self.yaw_rad.cos());

        let r_z = [[cy, -sy, 0.0], [sy, cy, 0.0], [0.0, 0.0, 1.0]];
        let r_x = [[1.0, 0.0, 0.0], [0.0, cp, -sp], [0.0, sp, cp]];
        let r_base = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]];
        let r_roll = [[cr, -sr, 0.0], [sr, cr, 0.0], [0.0, 0.0, 1.0]];

        mat3_mult(mat3_mult(mat3_mult(r_z, r_x), r_base), r_roll)
    }
}

impl CameraPose {
    /// Transform a unit camera-frame ray into ENU coordinates.
    pub fn transform_ray(&self, ray_cam: [f64; 3]) -> [f64; 3] {
        let r = self.rotation_matrix();
        [
            r[0][0] * ray_cam[0] + r[0][1] * ray_cam[1] + r[0][2] * ray_cam[2],
            r[1][0] * ray_cam[0] + r[1][1] * ray_cam[1] + r[1][2] * ray_cam[2],
            r[2][0] * ray_cam[0] + r[2][1] * ray_cam[1] + r[2][2] * ray_cam[2],
        ]
    }
}

/// Intersect camera-frame ray with horizontal ground plane at `ground_up_m`.
pub fn intersect_ground(
    ray_cam: [f64; 3],
    pose: &CameraPose,
    ground_up_m: f64,
) -> Result<[f64; 3], CameraError> {
    let ray_enu = pose.transform_ray(ray_cam);
    if ray_enu[2] >= -1e-6 {
        return Err(CameraError::RayAboveHorizon);
    }
    let delta_h = ground_up_m - pose.cam_up_m;
    if delta_h >= 0.0 {
        return Err(CameraError::InvalidParameters);
    }
    let t = delta_h / ray_enu[2];
    Ok([t * ray_enu[0], t * ray_enu[1], ground_up_m])
}

fn diff_ground_u_v(
    cam: &Intrinsics,
    pose: &CameraPose,
    u: f64,
    v: f64,
    p0: [f64; 3],
    g_up: f64,
) -> Result<([f64; 2], [f64; 2]), CameraError> {
    let p_u = intersect_ground(cam.pixel_to_ray(u + 0.5, v)?, pose, g_up)?;
    let p_v = intersect_ground(cam.pixel_to_ray(u, v + 0.5)?, pose, g_up)?;
    Ok((
        [2.0 * (p_u[0] - p0[0]), 2.0 * (p_u[1] - p0[1])],
        [2.0 * (p_v[0] - p0[0]), 2.0 * (p_v[1] - p0[1])],
    ))
}

fn diff_ground_attitude(
    ray: [f64; 3],
    pose: &CameraPose,
    p0: [f64; 3],
    g_up: f64,
) -> Result<([f64; 2], [f64; 2]), CameraError> {
    let eps = 1e-4;
    let mut p_p = *pose;
    p_p.pitch_rad += eps;
    let pt_p = intersect_ground(ray, &p_p, g_up)?;
    let mut p_r = *pose;
    p_r.roll_rad += eps;
    let pt_r = intersect_ground(ray, &p_r, g_up)?;
    Ok((
        [(pt_p[0] - p0[0]) / eps, (pt_p[1] - p0[1]) / eps],
        [(pt_r[0] - p0[0]) / eps, (pt_r[1] - p0[1]) / eps],
    ))
}
