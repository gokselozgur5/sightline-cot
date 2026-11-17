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
