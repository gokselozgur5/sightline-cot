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

/// Brown-Conrady lens distortion with radial (k1, k2, k3) and tangential (p1, p2) terms.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BrownConrady {
    pub k1: f64,
    pub k2: f64,
    pub k3: f64,
    pub p1: f64,
    pub p2: f64,
}

impl BrownConrady {
    /// Apply distortion to normalized image coordinates (x, y).
    pub fn distort(&self, x: f64, y: f64) -> (f64, f64) {
        let r2 = x * x + y * y;
        let radial = 1.0 + r2 * (self.k1 + r2 * (self.k2 + r2 * self.k3));
        let xd = x * radial + 2.0 * self.p1 * x * y + self.p2 * (r2 + 2.0 * x * x);
        let yd = y * radial + self.p1 * (r2 + 2.0 * y * y) + 2.0 * self.p2 * x * y;
        (xd, yd)
    }
}

impl BrownConrady {
    /// Analytic Jacobian of `distort` as rows [d xd, d yd] over columns [d x, d y].
    pub fn jacobian(&self, x: f64, y: f64) -> [[f64; 2]; 2] {
        let r2 = x * x + y * y;
        let radial = 1.0 + r2 * (self.k1 + r2 * (self.k2 + r2 * self.k3));
        let g = 2.0 * (self.k1 + r2 * (2.0 * self.k2 + 3.0 * r2 * self.k3));
        let cross = g * x * y + 2.0 * self.p1 * x + 2.0 * self.p2 * y;
        let dxx = radial + g * x * x + 2.0 * self.p1 * y + 6.0 * self.p2 * x;
        let dyy = radial + g * y * y + 6.0 * self.p1 * y + 2.0 * self.p2 * x;
        [[dxx, cross], [cross, dyy]]
    }
}

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
                if theta.is_finite() && (0.0..std::f64::consts::PI).contains(&theta) {
                    return Ok(theta);
                } else {
                    return Err(CameraError::DistortionSolverFailed);
                }
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
        if !u.is_finite()
            || !v.is_finite()
            || !fx.is_finite()
            || !fy.is_finite()
            || fx <= 0.0
            || fy <= 0.0
        {
            return Err(CameraError::InvalidParameters);
        }
        let x = (u - cx) / fx;
        let y = (v - cy) / fy;
        let norm = (x * x + y * y + 1.0).sqrt();
        Ok([x / norm, y / norm, 1.0 / norm])
    }
}

impl Intrinsics {
    #[allow(clippy::too_many_arguments)]
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
        if !u.is_finite()
            || !v.is_finite()
            || !fx.is_finite()
            || !fy.is_finite()
            || fx <= 0.0
            || fy <= 0.0
        {
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

#[allow(clippy::needless_range_loop)]
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
        let r_base = [[1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, -1.0]];
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
    if !ray_cam.iter().all(|x| x.is_finite())
        || !pose.roll_rad.is_finite()
        || !pose.pitch_rad.is_finite()
        || !pose.yaw_rad.is_finite()
        || !pose.cam_up_m.is_finite()
        || !ground_up_m.is_finite()
    {
        return Err(CameraError::InvalidParameters);
    }
    let ray_enu = pose.transform_ray(ray_cam);
    if !ray_enu[2].is_finite() || ray_enu[2] >= -1e-6 {
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

#[allow(clippy::type_complexity)]
fn diff_ground_attitude(
    ray: [f64; 3],
    pose: &CameraPose,
    p0: [f64; 3],
    g_up: f64,
) -> Result<([f64; 2], [f64; 2], [f64; 2]), CameraError> {
    let eps = 1e-4;
    let mut p_p = *pose;
    p_p.pitch_rad += eps;
    let pt_p = intersect_ground(ray, &p_p, g_up)?;
    let mut p_r = *pose;
    p_r.roll_rad += eps;
    let pt_r = intersect_ground(ray, &p_r, g_up)?;
    let mut p_y = *pose;
    p_y.yaw_rad += eps;
    let pt_y = intersect_ground(ray, &p_y, g_up)?;
    Ok((
        [(pt_p[0] - p0[0]) / eps, (pt_p[1] - p0[1]) / eps],
        [(pt_r[0] - p0[0]) / eps, (pt_r[1] - p0[1]) / eps],
        [(pt_y[0] - p0[0]) / eps, (pt_y[1] - p0[1]) / eps],
    ))
}

/// Estimate 90% Circular Error (CE90) and Linear Error (LE90) in meters.
pub fn estimate_covariance_ce_le(
    cam: &Intrinsics,
    pose: &CameraPose,
    u: f64,
    v: f64,
    g_up: f64,
    px_sig: f64,
    att_sig: f64,
) -> Result<(f64, f64), CameraError> {
    if !px_sig.is_finite() || px_sig < 0.0 || !att_sig.is_finite() || att_sig < 0.0 {
        return Err(CameraError::InvalidParameters);
    }
    let ray = cam.pixel_to_ray(u, v)?;
    let p0 = intersect_ground(ray, pose, g_up)?;
    let (d_u, d_v) = diff_ground_u_v(cam, pose, u, v, p0, g_up)?;
    let (d_p, d_r, d_y) = diff_ground_attitude(ray, pose, p0, g_up)?;
    let (v_px, v_att) = (px_sig * px_sig, att_sig * att_sig);
    let var_e = (d_u[0].powi(2) + d_v[0].powi(2)) * v_px
        + (d_p[0].powi(2) + d_r[0].powi(2) + d_y[0].powi(2)) * v_att;
    let var_n = (d_u[1].powi(2) + d_v[1].powi(2)) * v_px
        + (d_p[1].powi(2) + d_r[1].powi(2) + d_y[1].powi(2)) * v_att;
    let ce90 = 2.146 * (0.5 * (var_e + var_n)).sqrt();
    let r_enu = pose.transform_ray(ray);
    let var_u = ((pose.cam_up_m - g_up) * (d_u[1].abs() * px_sig + d_p[1].abs() * att_sig)
        / (-r_enu[2]).max(1e-3))
    .powi(2)
        * 1e-4
        + 0.25;
    let le90 = 1.6449 * var_u.sqrt();
    Ok((ce90, le90))
}

const WGS84_A: f64 = 6378137.0;
const WGS84_F: f64 = 1.0 / 298.257223563;
const WGS84_B: f64 = WGS84_A * (1.0 - WGS84_F);
const WGS84_E2: f64 = 2.0 * WGS84_F - WGS84_F * WGS84_F;
const WGS84_E_PRIME2: f64 = (WGS84_A * WGS84_A - WGS84_B * WGS84_B) / (WGS84_B * WGS84_B);

fn geodetic_to_ecef(lat_deg: f64, lon_deg: f64, hae_m: f64) -> [f64; 3] {
    let lat_rad = lat_deg.to_radians();
    let lon_rad = lon_deg.to_radians();
    let (s_lat, c_lat) = (lat_rad.sin(), lat_rad.cos());
    let (s_lon, c_lon) = (lon_rad.sin(), lon_rad.cos());
    let n = WGS84_A / (1.0 - WGS84_E2 * s_lat * s_lat).sqrt();
    let x = (n + hae_m) * c_lat * c_lon;
    let y = (n + hae_m) * c_lat * s_lon;
    let z = (n * (1.0 - WGS84_E2) + hae_m) * s_lat;
    [x, y, z]
}

fn enu_to_ecef(enu_m: [f64; 3], ref_lat: f64, ref_lon: f64, ref_hae: f64) -> [f64; 3] {
    let ref_ecef = geodetic_to_ecef(ref_lat, ref_lon, ref_hae);
    let (s_lat, c_lat) = (ref_lat.to_radians().sin(), ref_lat.to_radians().cos());
    let (s_lon, c_lon) = (ref_lon.to_radians().sin(), ref_lon.to_radians().cos());
    let (e, n, u) = (enu_m[0], enu_m[1], enu_m[2]);
    let dx = -s_lon * e - s_lat * c_lon * n + c_lat * c_lon * u;
    let dy = c_lon * e - s_lat * s_lon * n + c_lat * s_lon * u;
    let dz = c_lat * n + s_lat * u;
    [ref_ecef[0] + dx, ref_ecef[1] + dy, ref_ecef[2] + dz]
}

fn ecef_to_geodetic(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let p = (x * x + y * y).sqrt();
    let lon_rad = y.atan2(x);
    let theta = (z * WGS84_A).atan2(p * WGS84_B);
    let (s_th, c_th) = (theta.sin(), theta.cos());
    let num = z + WGS84_E_PRIME2 * WGS84_B * s_th * s_th * s_th;
    let den = p - WGS84_E2 * WGS84_A * c_th * c_th * c_th;
    let lat_rad = num.atan2(den);
    let s_lat = lat_rad.sin();
    let n = WGS84_A / (1.0 - WGS84_E2 * s_lat * s_lat).sqrt();
    let hae_m = if p > 1e-6 {
        p / lat_rad.cos() - n
    } else {
        z.abs() - WGS84_B
    };
    (lat_rad.to_degrees(), lon_rad.to_degrees(), hae_m)
}

/// Convert local ENU offset from reference WGS84 origin to Geodetic (lat, lon, hae).
pub fn enu_to_geodetic(
    enu_m: [f64; 3],
    ref_lat_deg: f64,
    ref_lon_deg: f64,
    ref_hae_m: f64,
) -> (f64, f64, f64) {
    let ecef = enu_to_ecef(enu_m, ref_lat_deg, ref_lon_deg, ref_hae_m);
    ecef_to_geodetic(ecef[0], ecef[1], ecef[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinhole_nadir_projection() {
        let cam = Intrinsics::Pinhole {
            fx: 1000.0,
            fy: 1000.0,
            cx: 500.0,
            cy: 500.0,
        };
        let ray = cam.pixel_to_ray(500.0, 500.0).unwrap();
        assert!((ray[0]).abs() < 1e-9);
        assert!((ray[1]).abs() < 1e-9);
        assert!((ray[2] - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_fisheye_inversion_and_center() {
        let fish = Intrinsics::EquidistantFisheye {
            fx: 800.0,
            fy: 800.0,
            cx: 400.0,
            cy: 400.0,
            k1: 0.05,
            k2: -0.01,
        };
        let ray_c = fish.pixel_to_ray(400.0, 400.0).unwrap();
        assert!((ray_c[2] - 1.0).abs() < 1e-9);
        let ray_off = fish.pixel_to_ray(500.0, 500.0).unwrap();
        let len = (ray_off[0].powi(2) + ray_off[1].powi(2) + ray_off[2].powi(2)).sqrt();
        assert!((len - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_nadir_ground_intersection() {
        let pose = CameraPose {
            roll_rad: 0.0,
            pitch_rad: 0.0,
            yaw_rad: 0.0,
            cam_up_m: 100.0,
        };
        let ray_cam = [0.0, 0.0, 1.0];
        let pt = intersect_ground(ray_cam, &pose, 0.0).unwrap();
        assert!(pt[0].abs() < 1e-9);
        assert!(pt[1].abs() < 1e-9);
        assert!(pt[2].abs() < 1e-9);
    }

    #[test]
    fn test_oblique_and_horizon_rejection() {
        let pose = CameraPose {
            roll_rad: 0.0,
            pitch_rad: std::f64::consts::FRAC_PI_4,
            yaw_rad: 0.0,
            cam_up_m: 100.0,
        };
        let pt = intersect_ground([0.0, 0.0, 1.0], &pose, 0.0).unwrap();
        assert!((pt[1] - 100.0).abs() < 1e-4);
        let res_up = intersect_ground([0.0, -1.0, 0.0], &pose, 0.0);
        assert_eq!(res_up, Err(CameraError::RayAboveHorizon));
    }

    #[test]
    fn test_wgs84_round_trip() {
        let (lat0, lon0, hae0) = (37.7749, -122.4194, 50.0);
        let enu_in = [150.0, -200.0, 10.0];
        let (lat, lon, hae) = enu_to_geodetic(enu_in, lat0, lon0, hae0);
        let ecef_orig = enu_to_ecef(enu_in, lat0, lon0, hae0);
        let ecef_rt = geodetic_to_ecef(lat, lon, hae);
        let err = ((ecef_orig[0] - ecef_rt[0]).powi(2)
            + (ecef_orig[1] - ecef_rt[1]).powi(2)
            + (ecef_orig[2] - ecef_rt[2]).powi(2))
        .sqrt();
        assert!(err < 1e-4);
    }

    #[test]
    fn test_ce90_le90_growth_with_slant_range() {
        let cam = Intrinsics::Pinhole {
            fx: 1000.0,
            fy: 1000.0,
            cx: 500.0,
            cy: 500.0,
        };
        let pose1 = CameraPose {
            roll_rad: 0.0,
            pitch_rad: 0.2,
            yaw_rad: 0.0,
            cam_up_m: 50.0,
        };
        let pose2 = CameraPose {
            roll_rad: 0.0,
            pitch_rad: 0.2,
            yaw_rad: 0.0,
            cam_up_m: 200.0,
        };
        let (ce1, le1) =
            estimate_covariance_ce_le(&cam, &pose1, 500.0, 500.0, 0.0, 1.0, 0.001).unwrap();
        let (ce2, le2) =
            estimate_covariance_ce_le(&cam, &pose2, 500.0, 500.0, 0.0, 1.0, 0.001).unwrap();
        assert!(ce2 > ce1);
        assert!(le2 > le1);
    }

    #[test]
    fn test_so3_determinant() {
        let pose = CameraPose {
            roll_rad: 0.15,
            pitch_rad: 0.35,
            yaw_rad: -0.7,
            cam_up_m: 120.0,
        };
        let r = pose.rotation_matrix();
        let det = r[0][0] * (r[1][1] * r[2][2] - r[1][2] * r[2][1])
            - r[0][1] * (r[1][0] * r[2][2] - r[1][2] * r[2][0])
            + r[0][2] * (r[1][0] * r[2][1] - r[1][1] * r[2][0]);
        assert!((det - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_so3_orthogonality() {
        let pose = CameraPose {
            roll_rad: -0.2,
            pitch_rad: 0.4,
            yaw_rad: 1.1,
            cam_up_m: 80.0,
        };
        let r = pose.rotation_matrix();
        for i in 0..3 {
            for j in 0..3 {
                let dot = (0..3).fold(0.0, |acc, k| acc + r[i][k] * r[j][k]);
                let exp = if i == j { 1.0 } else { 0.0 };
                assert!((dot - exp).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn test_fisheye_forward_inverse_round_trip() {
        let (fx, fy, cx, cy) = (800.0, 800.0, 400.0, 400.0);
        let (k1, k2) = (0.05, -0.01);
        let fish = Intrinsics::EquidistantFisheye {
            fx,
            fy,
            cx,
            cy,
            k1,
            k2,
        };
        let (u_in, v_in) = (450.0, 430.0);
        let ray = fish.pixel_to_ray(u_in, v_in).unwrap();
        let theta = ray[2].acos();
        let sin_th = theta.sin();
        let r_d = theta * (1.0 + k1 * theta.powi(2) + k2 * theta.powi(4));
        let u_out = cx + fx * (ray[0] / sin_th) * r_d;
        let v_out = cy + fy * (ray[1] / sin_th) * r_d;
        assert!((u_out - u_in).abs() < 1e-9);
        assert!((v_out - v_in).abs() < 1e-9);
    }

    #[test]
    fn test_nan_and_pole_rejection() {
        let cam = Intrinsics::Pinhole {
            fx: 1000.0,
            fy: 1000.0,
            cx: 500.0,
            cy: 500.0,
        };
        let res_nan = cam.pixel_to_ray(f64::NAN, 500.0);
        assert_eq!(res_nan, Err(CameraError::InvalidParameters));

        let (lat_p, _lon_p, hae_p) = enu_to_geodetic([0.0, 0.0, 10.0], 90.0, 0.0, 0.0);
        assert!((lat_p - 90.0).abs() < 1e-4);
        assert!((hae_p - 10.0).abs() < 1e-3);
    }
}

/// Regular 2D Digital Elevation Model (DEM) grid.
#[derive(Debug, Clone, PartialEq)]
pub struct DemGrid {
    pub origin_e_m: f64,
    pub origin_n_m: f64,
    pub cell_step_m: f64,
    pub cols: usize,
    pub rows: usize,
    pub elevations: Vec<f64>,
}

impl DemGrid {
    /// Construct a validated DemGrid.
    pub fn new(
        e0: f64,
        n0: f64,
        step: f64,
        cols: usize,
        rows: usize,
        elevations: Vec<f64>,
    ) -> Result<Self, CameraError> {
        if !e0.is_finite() || !n0.is_finite() || !step.is_finite() || step <= 0.0 {
            return Err(CameraError::InvalidParameters);
        }
        if cols < 2 || rows < 2 || elevations.len() != cols * rows {
            return Err(CameraError::InvalidParameters);
        }
        if !elevations.iter().all(|h| h.is_finite()) {
            return Err(CameraError::InvalidParameters);
        }
        Ok(Self {
            origin_e_m: e0,
            origin_n_m: n0,
            cell_step_m: step,
            cols,
            rows,
            elevations,
        })
    }

    /// Check if local ENU coordinates fall within the grid bounds.
    pub fn contains_en(&self, e_m: f64, n_m: f64) -> bool {
        let max_e = self.origin_e_m + (self.cols - 1) as f64 * self.cell_step_m;
        let max_n = self.origin_n_m + (self.rows - 1) as f64 * self.cell_step_m;
        e_m >= self.origin_e_m && e_m <= max_e && n_m >= self.origin_n_m && n_m <= max_n
    }

    /// Bilinear elevation lookup at (e_m, n_m).
    pub fn elevation_at(&self, e_m: f64, n_m: f64) -> Option<f64> {
        if !self.contains_en(e_m, n_m) {
            return None;
        }
        let u = ((e_m - self.origin_e_m) / self.cell_step_m).max(0.0);
        let v = ((n_m - self.origin_n_m) / self.cell_step_m).max(0.0);
        let c0 = (u.floor() as usize).min(self.cols - 2);
        let r0 = (v.floor() as usize).min(self.rows - 2);
        let s = u - c0 as f64;
        let t = v - r0 as f64;
        let z00 = self.elevations[r0 * self.cols + c0];
        let z10 = self.elevations[r0 * self.cols + c0 + 1];
        let z01 = self.elevations[(r0 + 1) * self.cols + c0];
        let z11 = self.elevations[(r0 + 1) * self.cols + c0 + 1];
        Some((1.0 - s) * (1.0 - t) * z00 + s * (1.0 - t) * z10 + (1.0 - s) * t * z01 + s * t * z11)
    }

    /// Intersect camera ray with DEM surface via bounded iterative step.
    pub fn intersect_dem(
        &self,
        ray_cam: [f64; 3],
        pose: &CameraPose,
        max_iters: usize,
    ) -> Result<[f64; 3], CameraError> {
        let ray_enu = pose.transform_ray(ray_cam);
        if ray_enu[2] >= -1e-6 {
            return Err(CameraError::RayAboveHorizon);
        }
        let mut t = (0.0 - pose.cam_up_m) / ray_enu[2];
        for _ in 0..max_iters {
            let (e, n) = (t * ray_enu[0], t * ray_enu[1]);
            let h = self
                .elevation_at(e, n)
                .ok_or(CameraError::RayAboveHorizon)?;
            let diff = pose.cam_up_m + t * ray_enu[2] - h;
            if diff.abs() < 1e-3 {
                return Ok([e, n, h]);
            }
            t += diff / (-ray_enu[2]);
        }
        let (e, n) = (t * ray_enu[0], t * ray_enu[1]);
        self.elevation_at(e, n)
            .map(|h| [e, n, h])
            .ok_or(CameraError::RayAboveHorizon)
    }
}

#[cfg(test)]
mod dem_tests {
    use super::*;

    #[test]
    fn test_dem_bilinear_and_slope_intersection() {
        let grid = DemGrid::new(
            0.0,
            0.0,
            10.0,
            3,
            3,
            vec![0.0, 2.0, 4.0, 0.0, 2.0, 4.0, 0.0, 2.0, 4.0],
        )
        .unwrap();
        assert_eq!(grid.elevation_at(5.0, 5.0), Some(1.0));
        let pose = CameraPose {
            roll_rad: 0.0,
            pitch_rad: 0.0,
            yaw_rad: 0.0,
            cam_up_m: 100.0,
        };
        let pt = grid.intersect_dem([0.0, 0.0, 1.0], &pose, 10).unwrap();
        assert!((pt[2] - 0.0).abs() < 1e-2);
    }
}
