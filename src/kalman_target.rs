//! Tactical target velocity vector Kalman prediction filter.

/// Error types for tactical Kalman filtering and state prediction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KalmanTargetError {
    /// Non-finite numerical state or measurement input.
    NonFiniteInput,
    /// Invalid time delta (negative or non-positive when required).
    InvalidDeltaTime,
    /// Matrix inversion failure due to singularity or degeneracy.
    SingularMatrix,
    /// Measurement rejected by residual gating threshold.
    GatedMeasurement { residual: f64, threshold: f64 },
    /// Time delta exceeds configured maximum coast limit.
    CoastLimitExceeded { dt_sec: f64, max_coast_sec: f64 },
}

impl std::fmt::Display for KalmanTargetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteInput => {
                write!(f, "non-finite input encountered in Kalman target filter")
            }
            Self::InvalidDeltaTime => write!(f, "invalid delta time (must be positive and finite)"),
            Self::SingularMatrix => {
                write!(f, "singular or ill-conditioned matrix inversion failure")
            }
            Self::GatedMeasurement {
                residual,
                threshold,
            } => {
                write!(
                    f,
                    "measurement gated out: residual {residual} > gate {threshold}"
                )
            }
            Self::CoastLimitExceeded {
                dt_sec,
                max_coast_sec,
            } => {
                write!(f, "coast limit exceeded: {dt_sec}s > {max_coast_sec}s")
            }
        }
    }
}

impl std::error::Error for KalmanTargetError {}

/// Tuning and validation configuration for tactical target filter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TargetFilterConfig {
    /// Continuous-time acceleration spectral density q (m^2 / s^3).
    pub process_noise_q: f64,
    /// Default position measurement variance (m^2).
    pub default_pos_var: f64,
    /// Default velocity measurement variance (m^2 / s^2).
    pub default_vel_var: f64,
    /// Maximum allowed extrapolation coasting duration (seconds).
    pub max_coast_sec: f64,
    /// Mahalanobis gating threshold (chi-squared gate value).
    pub gate_threshold: f64,
}

impl Default for TargetFilterConfig {
    fn default() -> Self {
        Self {
            process_noise_q: 1.0,
            default_pos_var: 25.0,
            default_vel_var: 4.0,
            max_coast_sec: 60.0,
            gate_threshold: 16.0,
        }
    }
}

impl TargetFilterConfig {
    /// Validate filter configuration values.
    pub fn validate(&self) -> Result<(), KalmanTargetError> {
        let valid = self.process_noise_q.is_finite()
            && self.process_noise_q > 0.0
            && self.default_pos_var.is_finite()
            && self.default_pos_var > 0.0
            && self.default_vel_var.is_finite()
            && self.default_vel_var > 0.0
            && self.max_coast_sec.is_finite()
            && self.max_coast_sec > 0.0
            && self.gate_threshold.is_finite()
            && self.gate_threshold > 0.0;
        if valid {
            Ok(())
        } else {
            Err(KalmanTargetError::NonFiniteInput)
        }
    }
}

/// 3D Kinematic state representing tactical target position and velocity.
#[derive(Debug, Clone, PartialEq)]
pub struct KinematicState {
    /// State vector: [x, y, z, vx, vy, vz] in ENU coordinates.
    pub x: [f64; 6],
    /// 6x6 State error covariance matrix.
    pub cov: [[f64; 6]; 6],
    /// Epoch timestamp in milliseconds.
    pub timestamp_ms: i64,
}

impl KinematicState {
    /// Create a new validated kinematic state.
    pub fn new(
        x: [f64; 6],
        cov: [[f64; 6]; 6],
        timestamp_ms: i64,
    ) -> Result<Self, KalmanTargetError> {
        for v in x.iter() {
            if !v.is_finite() {
                return Err(KalmanTargetError::NonFiniteInput);
            }
        }
        for row in cov.iter() {
            for v in row.iter() {
                if !v.is_finite() {
                    return Err(KalmanTargetError::NonFiniteInput);
                }
            }
        }
        Ok(Self {
            x,
            cov,
            timestamp_ms,
        })
    }

    /// Extract 3D position [x, y, z] in meters.
    #[inline]
    pub fn position(&self) -> [f64; 3] {
        [self.x[0], self.x[1], self.x[2]]
    }

    /// Extract 3D velocity vector [vx, vy, vz] in meters per second.
    #[inline]
    pub fn velocity(&self) -> [f64; 3] {
        [self.x[3], self.x[4], self.x[5]]
    }

    /// Compute 3D speed magnitude in meters per second.
    #[inline]
    pub fn speed_mps(&self) -> f64 {
        let (vx, vy, vz) = (self.x[3], self.x[4], self.x[5]);
        (vx * vx + vy * vy + vz * vz).sqrt()
    }

    /// Compute horizontal ground speed in meters per second.
    #[inline]
    pub fn ground_speed_mps(&self) -> f64 {
        self.x[3].hypot(self.x[4])
    }

    /// Compute horizontal course heading in degrees clockwise from North [0, 360).
    pub fn course_deg(&self) -> f64 {
        let (vx, vy) = (self.x[3], self.x[4]);
        if vx == 0.0 && vy == 0.0 {
            0.0
        } else {
            vx.atan2(vy).to_degrees().rem_euclid(360.0)
        }
    }

    /// Compute vertical elevation climb/dive angle in degrees [-90, +90].
    pub fn elevation_deg(&self) -> f64 {
        let gs = self.ground_speed_mps();
        if gs == 0.0 && self.x[5] == 0.0 {
            0.0
        } else {
            self.x[5].atan2(gs).to_degrees()
        }
    }

    /// Compute estimated Circular Error Probable (CEP-50) in meters.
    pub fn position_uncertainty_cep(&self) -> f64 {
        let var_x = self.cov[0][0].max(0.0);
        let var_y = self.cov[1][1].max(0.0);
        1.1774 * (0.5 * (var_x + var_y)).sqrt()
    }

    /// Compute 1-sigma velocity root-mean-square uncertainty in m/s.
    pub fn velocity_uncertainty_1sigma(&self) -> f64 {
        let var_vx = self.cov[3][3].max(0.0);
        let var_vy = self.cov[4][4].max(0.0);
        let var_vz = self.cov[5][5].max(0.0);
        ((var_vx + var_vy + var_vz) / 3.0).sqrt()
    }

    /// Extrapolate kinematic state forward by time delta dt_sec.
    pub fn extrapolate(&self, dt_sec: f64, q: f64) -> Result<Self, KalmanTargetError> {
        if !dt_sec.is_finite() || dt_sec < 0.0 {
            return Err(KalmanTargetError::InvalidDeltaTime);
        }
        if !q.is_finite() || q < 0.0 {
            return Err(KalmanTargetError::NonFiniteInput);
        }
        let (x_pred, cov_pred) = predict_step(&self.x, &self.cov, dt_sec, q);
        let dt_ms = (dt_sec * 1000.0).round() as i64;
        Ok(Self {
            x: x_pred,
            cov: cov_pred,
            timestamp_ms: self.timestamp_ms + dt_ms,
        })
    }
}

/// Compute state and covariance prediction step under continuous white noise acceleration.
fn predict_step(x: &[f64; 6], cov: &[[f64; 6]; 6], dt: f64, q: f64) -> ([f64; 6], [[f64; 6]; 6]) {
    let mut x_pred = *x;
    x_pred[0] += dt * x[3];
    x_pred[1] += dt * x[4];
    x_pred[2] += dt * x[5];

    let mut f = [[0.0; 6]; 6];
    for i in 0..6 {
        f[i][i] = 1.0;
    }
    f[0][3] = dt;
    f[1][4] = dt;
    f[2][5] = dt;

    let (q1, q2, q3) = (q * dt.powi(3) / 3.0, q * dt.powi(2) / 2.0, q * dt);
    let mut q_mat = [[0.0; 6]; 6];
    for k in 0..3 {
        q_mat[k][k] = q1;
        q_mat[k][k + 3] = q2;
        q_mat[k + 3][k] = q2;
        q_mat[k + 3][k + 3] = q3;
    }

    let mut f_p = [[0.0; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            let mut sum = 0.0;
            for k in 0..6 {
                sum += f[i][k] * cov[k][j];
            }
            f_p[i][j] = sum;
        }
    }

    let mut cov_pred = q_mat;
    for i in 0..6 {
        for j in 0..6 {
            let mut sum = 0.0;
            for k in 0..6 {
                sum += f_p[i][k] * f[j][k];
            }
            cov_pred[i][j] += sum;
        }
    }

    for i in 0..6 {
        for j in (i + 1)..6 {
            let sym = 0.5 * (cov_pred[i][j] + cov_pred[j][i]);
            cov_pred[i][j] = sym;
            cov_pred[j][i] = sym;
        }
    }

    (x_pred, cov_pred)
}

/// Invert a 3x3 real symmetric matrix.
fn invert_3x3(m: &[[f64; 3]; 3]) -> Result<[[f64; 3]; 3], KalmanTargetError> {
    let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
    if !det.is_finite() || det.abs() < 1e-15 {
        return Err(KalmanTargetError::SingularMatrix);
    }
    let inv_det = 1.0 / det;
    Ok([
        [
            (m[1][1] * m[2][2] - m[1][2] * m[2][1]) * inv_det,
            (m[0][2] * m[2][1] - m[0][1] * m[2][2]) * inv_det,
            (m[0][1] * m[1][2] - m[0][2] * m[1][1]) * inv_det,
        ],
        [
            (m[1][2] * m[2][0] - m[1][0] * m[2][2]) * inv_det,
            (m[0][0] * m[2][2] - m[0][2] * m[2][0]) * inv_det,
            (m[0][2] * m[1][0] - m[0][0] * m[1][2]) * inv_det,
        ],
        [
            (m[1][0] * m[2][1] - m[1][1] * m[2][0]) * inv_det,
            (m[0][1] * m[2][0] - m[0][0] * m[2][1]) * inv_det,
            (m[0][0] * m[1][1] - m[0][1] * m[1][0]) * inv_det,
        ],
    ])
}

/// Perform 3D linear Kalman measurement update with Joseph form covariance.
fn update_3d(
    x: &mut [f64; 6],
    cov: &mut [[f64; 6]; 6],
    z: [f64; 3],
    r_diag: [f64; 3],
    offset: usize,
    gate_threshold: f64,
) -> Result<f64, KalmanTargetError> {
    for &val in &z {
        if !val.is_finite() {
            return Err(KalmanTargetError::NonFiniteInput);
        }
    }
    for &val in &r_diag {
        if !val.is_finite() || val <= 0.0 {
            return Err(KalmanTargetError::NonFiniteInput);
        }
    }

    let y = [z[0] - x[offset], z[1] - x[offset + 1], z[2] - x[offset + 2]];
    let mut s = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            s[i][j] = cov[offset + i][offset + j];
        }
        s[i][i] += r_diag[i];
    }

    let s_inv = invert_3x3(&s)?;
    let mut mahalanobis_sq = 0.0;
    for i in 0..3 {
        let mut term = 0.0;
        for j in 0..3 {
            term += s_inv[i][j] * y[j];
        }
        mahalanobis_sq += y[i] * term;
    }

    if mahalanobis_sq > gate_threshold {
        return Err(KalmanTargetError::GatedMeasurement {
            residual: mahalanobis_sq,
            threshold: gate_threshold,
        });
    }

    let mut k_gain = [[0.0; 3]; 6];
    for i in 0..6 {
        for j in 0..3 {
            let mut sum = 0.0;
            for m in 0..3 {
                sum += cov[i][offset + m] * s_inv[m][j];
            }
            k_gain[i][j] = sum;
        }
    }

    for i in 0..6 {
        let mut corr = 0.0;
        for j in 0..3 {
            corr += k_gain[i][j] * y[j];
        }
        x[i] += corr;
    }

    let mut a = [[0.0; 6]; 6];
    for i in 0..6 {
        a[i][i] = 1.0;
        for j in 0..3 {
            a[i][offset + j] -= k_gain[i][j];
        }
    }

    let mut a_p = [[0.0; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            let mut sum = 0.0;
            for m in 0..6 {
                sum += a[i][m] * cov[m][j];
            }
            a_p[i][j] = sum;
        }
    }

    let mut new_cov = [[0.0; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            let mut sum = 0.0;
            for m in 0..6 {
                sum += a_p[i][m] * a[j][m];
            }
            new_cov[i][j] = sum;
        }
    }

    for i in 0..6 {
        for j in 0..6 {
            let mut k_r_kt = 0.0;
            for m in 0..3 {
                k_r_kt += k_gain[i][m] * r_diag[m] * k_gain[j][m];
            }
            new_cov[i][j] += k_r_kt;
        }
    }

    for i in 0..6 {
        for j in (i + 1)..6 {
            let sym = 0.5 * (new_cov[i][j] + new_cov[j][i]);
            new_cov[i][j] = sym;
            new_cov[j][i] = sym;
        }
    }

    *cov = new_cov;
    Ok(mahalanobis_sq)
}

/// Tactical target velocity vector Kalman prediction filter.
#[derive(Debug, Clone, PartialEq)]
pub struct TacticalTargetFilter {
    /// Unique tactical target or track identifier.
    pub target_id: String,
    /// Current estimated kinematic state.
    pub current_state: KinematicState,
    /// Operational filter configuration.
    pub config: TargetFilterConfig,
    /// Timestamp of most recent measurement update in milliseconds.
    pub last_update_ms: i64,
    /// Total count of successful measurement updates.
    pub update_count: usize,
    /// Consecutive extrapolation coast cycles since last update.
    pub coast_count: usize,
}

impl TacticalTargetFilter {
    /// Initialize a new tactical target Kalman filter.
    pub fn new(
        target_id: impl Into<String>,
        initial_pos: [f64; 3],
        initial_vel: [f64; 3],
        pos_var: f64,
        vel_var: f64,
        timestamp_ms: i64,
        config: TargetFilterConfig,
    ) -> Result<Self, KalmanTargetError> {
        config.validate()?;
        if !pos_var.is_finite() || pos_var <= 0.0 || !vel_var.is_finite() || vel_var <= 0.0 {
            return Err(KalmanTargetError::NonFiniteInput);
        }

        let x = [
            initial_pos[0],
            initial_pos[1],
            initial_pos[2],
            initial_vel[0],
            initial_vel[1],
            initial_vel[2],
        ];

        let mut cov = [[0.0; 6]; 6];
        for i in 0..3 {
            cov[i][i] = pos_var;
            cov[i + 3][i + 3] = vel_var;
        }

        let state = KinematicState::new(x, cov, timestamp_ms)?;
        Ok(Self {
            target_id: target_id.into(),
            current_state: state,
            config,
            last_update_ms: timestamp_ms,
            update_count: 0,
            coast_count: 0,
        })
    }

    /// Advance target filter forward in time to the target timestamp.
    pub fn predict_to(&mut self, timestamp_ms: i64) -> Result<(), KalmanTargetError> {
        if timestamp_ms < self.current_state.timestamp_ms {
            return Err(KalmanTargetError::InvalidDeltaTime);
        }
        let dt_ms = timestamp_ms - self.current_state.timestamp_ms;
        if dt_ms == 0 {
            return Ok(());
        }
        let dt_sec = dt_ms as f64 / 1000.0;
        let time_since_update_sec = (timestamp_ms - self.last_update_ms) as f64 / 1000.0;
        if time_since_update_sec > self.config.max_coast_sec {
            return Err(KalmanTargetError::CoastLimitExceeded {
                dt_sec: time_since_update_sec,
                max_coast_sec: self.config.max_coast_sec,
            });
        }
        self.current_state = self
            .current_state
            .extrapolate(dt_sec, self.config.process_noise_q)?;
        self.coast_count += 1;
        Ok(())
    }

    /// Process 3D position measurement update with residual gating.
    pub fn update_position(
        &mut self,
        pos: [f64; 3],
        pos_var: Option<[f64; 3]>,
        timestamp_ms: i64,
    ) -> Result<f64, KalmanTargetError> {
        self.predict_to(timestamp_ms)?;
        let r_diag = pos_var.unwrap_or([
            self.config.default_pos_var,
            self.config.default_pos_var,
            self.config.default_pos_var,
        ]);
        let d_sq = update_3d(
            &mut self.current_state.x,
            &mut self.current_state.cov,
            pos,
            r_diag,
            0,
            self.config.gate_threshold,
        )?;
        self.last_update_ms = timestamp_ms;
        self.update_count += 1;
        self.coast_count = 0;
        Ok(d_sq)
    }

    /// Process 3D velocity measurement update with residual gating.
    pub fn update_velocity(
        &mut self,
        vel: [f64; 3],
        vel_var: Option<[f64; 3]>,
        timestamp_ms: i64,
    ) -> Result<f64, KalmanTargetError> {
        self.predict_to(timestamp_ms)?;
        let r_diag = vel_var.unwrap_or([
            self.config.default_vel_var,
            self.config.default_vel_var,
            self.config.default_vel_var,
        ]);
        let d_sq = update_3d(
            &mut self.current_state.x,
            &mut self.current_state.cov,
            vel,
            r_diag,
            3,
            self.config.gate_threshold,
        )?;
        self.last_update_ms = timestamp_ms;
        self.update_count += 1;
        self.coast_count = 0;
        Ok(d_sq)
    }

    /// Process full state (position and velocity) measurement update.
    pub fn update_full_state(
        &mut self,
        pos: [f64; 3],
        vel: [f64; 3],
        pos_var: Option<[f64; 3]>,
        vel_var: Option<[f64; 3]>,
        timestamp_ms: i64,
    ) -> Result<(f64, f64), KalmanTargetError> {
        let pos_d2 = self.update_position(pos, pos_var, timestamp_ms)?;
        let vel_d2 = self.update_velocity(vel, vel_var, timestamp_ms)?;
        Ok((pos_d2, vel_d2))
    }

    /// Forecast target kinematic trajectory waypoints over a forward time horizon.
    pub fn predict_horizon(
        &self,
        horizon_sec: f64,
        step_sec: f64,
    ) -> Result<Vec<KinematicState>, KalmanTargetError> {
        if !horizon_sec.is_finite()
            || horizon_sec <= 0.0
            || !step_sec.is_finite()
            || step_sec <= 0.0
        {
            return Err(KalmanTargetError::InvalidDeltaTime);
        }
        let steps = (horizon_sec / step_sec).round() as usize;
        let mut traj = Vec::with_capacity(steps);
        let mut state = self.current_state.clone();
        for _ in 0..steps {
            state = state.extrapolate(step_sec, self.config.process_noise_q)?;
            traj.push(state.clone());
        }
        Ok(traj)
    }

    /// Compute estimated intercept flight time in seconds for a pursuing platform.
    pub fn estimate_intercept_time(
        &self,
        pursuer_pos: [f64; 3],
        pursuer_speed_mps: f64,
    ) -> Option<f64> {
        if !pursuer_speed_mps.is_finite() || pursuer_speed_mps <= 0.0 {
            return None;
        }
        for &coord in &pursuer_pos {
            if !coord.is_finite() {
                return None;
            }
        }
        let (rx, ry, rz) = (
            self.current_state.x[0] - pursuer_pos[0],
            self.current_state.x[1] - pursuer_pos[1],
            self.current_state.x[2] - pursuer_pos[2],
        );
        let dist_sq = rx * rx + ry * ry + rz * rz;
        if dist_sq < 1e-12 {
            return Some(0.0);
        }

        let (vx, vy, vz) = (
            self.current_state.x[3],
            self.current_state.x[4],
            self.current_state.x[5],
        );
        let v_sq = vx * vx + vy * vy + vz * vz;
        let s_sq = pursuer_speed_mps * pursuer_speed_mps;

        let a = v_sq - s_sq;
        let b = 2.0 * (rx * vx + ry * vy + rz * vz);
        let c = dist_sq;

        if a.abs() < 1e-9 {
            if b.abs() < 1e-9 {
                return None;
            }
            let t = -c / b;
            return if t > 1e-6 { Some(t) } else { None };
        }

        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return None;
        }
        let sqrt_disc = disc.sqrt();
        let t1 = (-b - sqrt_disc) / (2.0 * a);
        let t2 = (-b + sqrt_disc) / (2.0 * a);

        let valid1 = t1 > 1e-6;
        let valid2 = t2 > 1e-6;

        match (valid1, valid2) {
            (true, true) => Some(t1.min(t2)),
            (true, false) => Some(t1),
            (false, true) => Some(t2),
            (false, false) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults_and_validation() {
        let cfg = TargetFilterConfig::default();
        assert!(cfg.validate().is_ok());

        let mut bad_cfg = cfg;
        bad_cfg.process_noise_q = -1.0;
        assert_eq!(
            bad_cfg.validate().unwrap_err(),
            KalmanTargetError::NonFiniteInput
        );
    }

    #[test]
    fn test_kinematic_state_getters_and_course() {
        let x = [100.0, 200.0, 50.0, 10.0, 0.0, 0.0];
        let cov = [[0.0; 6]; 6];
        let state = KinematicState::new(x, cov, 1000).unwrap();

        assert_eq!(state.position(), [100.0, 200.0, 50.0]);
        assert_eq!(state.velocity(), [10.0, 0.0, 0.0]);
        assert!((state.speed_mps() - 10.0).abs() < 1e-9);
        assert!((state.ground_speed_mps() - 10.0).abs() < 1e-9);
        assert!((state.course_deg() - 90.0).abs() < 1e-6);
        assert!((state.elevation_deg() - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_extrapolate_constant_velocity() {
        let x = [0.0, 0.0, 0.0, 10.0, 20.0, -5.0];
        let cov = [[1.0; 6]; 6];
        let state = KinematicState::new(x, cov, 0).unwrap();

        let pred = state.extrapolate(2.0, 0.5).unwrap();
        assert_eq!(pred.timestamp_ms, 2000);
        assert!((pred.x[0] - 20.0).abs() < 1e-9);
        assert!((pred.x[1] - 40.0).abs() < 1e-9);
        assert!((pred.x[2] - (-10.0)).abs() < 1e-9);
        assert!((pred.x[3] - 10.0).abs() < 1e-9);
    }

    #[test]
    fn test_covariance_growth_over_time() {
        let x = [0.0; 6];
        let mut cov = [[0.0; 6]; 6];
        for i in 0..6 {
            cov[i][i] = 1.0;
        }
        let state = KinematicState::new(x, cov, 0).unwrap();
        let pred = state.extrapolate(5.0, 2.0).unwrap();

        assert!(pred.cov[0][0] > 1.0);
        assert!(pred.cov[1][1] > 1.0);
        assert!(pred.cov[2][2] > 1.0);
        assert!(pred.position_uncertainty_cep() > state.position_uncertainty_cep());
    }

    #[test]
    fn test_filter_position_update_convergence() {
        let cfg = TargetFilterConfig::default();
        let mut filter = TacticalTargetFilter::new(
            "TGT-01",
            [0.0, 0.0, 100.0],
            [10.0, 0.0, 0.0],
            100.0,
            25.0,
            1000,
            cfg,
        )
        .unwrap();

        let initial_cep = filter.current_state.position_uncertainty_cep();
        let d2 = filter
            .update_position([10.5, 0.2, 100.1], None, 2000)
            .unwrap();
        assert!(d2 >= 0.0);
        let updated_cep = filter.current_state.position_uncertainty_cep();
        assert!(updated_cep < initial_cep);
        assert_eq!(filter.update_count, 1);
        assert_eq!(filter.coast_count, 0);
    }

    #[test]
    fn test_filter_velocity_update() {
        let cfg = TargetFilterConfig::default();
        let mut filter = TacticalTargetFilter::new(
            "TGT-02",
            [0.0, 0.0, 0.0],
            [5.0, 5.0, 0.0],
            25.0,
            16.0,
            0,
            cfg,
        )
        .unwrap();

        let d2 = filter.update_velocity([6.0, 4.0, 0.5], None, 1000).unwrap();
        assert!(d2 >= 0.0);
        assert!((filter.current_state.x[3] - 5.0).abs() > 0.0);
    }

    #[test]
    fn test_filter_full_state_update() {
        let cfg = TargetFilterConfig::default();
