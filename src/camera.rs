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
    let ray = cam.pixel_to_ray(u, v)?;
    let p0 = intersect_ground(ray, pose, g_up)?;
    let (d_u, d_v) = diff_ground_u_v(cam, pose, u, v, p0, g_up)?;
    let (d_p, d_r) = diff_ground_attitude(ray, pose, p0, g_up)?;
    let (v_px, v_att) = (px_sig * px_sig, att_sig * att_sig);
    let var_e =
        (d_u[0].powi(2) + d_v[0].powi(2)) * v_px + (d_p[0].powi(2) + d_r[0].powi(2)) * v_att;
    let var_n =
        (d_u[1].powi(2) + d_v[1].powi(2)) * v_px + (d_p[1].powi(2) + d_r[1].powi(2)) * v_att;
    let ce90 = 2.146 * (0.5 * (var_e + var_n)).sqrt();
    let slant = (p0[0].powi(2) + p0[1].powi(2) + (pose.cam_up_m - g_up).powi(2)).sqrt();
    let le90 = (1.6449 * 0.01 * slant).max(0.5);
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
    let hae_m = p / lat_rad.cos() - n;
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
        let res_up = intersect_ground([0.0, 1.0, 0.0], &pose, 0.0);
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
}
