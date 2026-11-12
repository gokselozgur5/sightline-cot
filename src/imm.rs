//! Interacting Multiple Model (IMM-EKF) Multi-Model Target Tracker.

/// Number of parallel kinematic models in IMM filter.
pub const NUM_MODELS: usize = 3;
/// Constant Velocity (CV) model index.
pub const MODEL_CV: usize = 0;
/// Constant Acceleration (CA) model index.
pub const MODEL_CA: usize = 1;
/// Coordinated Turn (CT) model index.
pub const MODEL_CT: usize = 2;

/// IMM configuration parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImmConfig {
    pub pos_noise_m: f64,
    pub cv_proc_noise: f64,
    pub ca_proc_noise: f64,
    pub ct_proc_noise: f64,
    pub turn_rate_rad_s: f64,
}

impl Default for ImmConfig {
    fn default() -> Self {
        Self {
            pos_noise_m: 2.0,
            cv_proc_noise: 0.5,
            ca_proc_noise: 2.5,
            ct_proc_noise: 1.5,
            turn_rate_rad_s: 0.5235987756, // 30 deg/s
        }
    }
}

/// Single 6-DoF kinematic Kalman sub-filter.
#[derive(Debug, Clone, PartialEq)]
pub struct SubFilter {
    pub state: [f64; 6],
    pub cov: [[f64; 6]; 6],
}

impl SubFilter {
    pub fn new(init_pos: [f64; 3], pos_var: f64, vel_var: f64) -> Self {
        let mut cov = [[0.0; 6]; 6];
        cov[0][0] = pos_var;
        cov[1][1] = pos_var;
        cov[2][2] = pos_var;
        cov[3][3] = vel_var;
        cov[4][4] = vel_var;
        cov[5][5] = vel_var;
        Self {
            state: [init_pos[0], init_pos[1], init_pos[2], 0.0, 0.0, 0.0],
            cov,
        }
    }
}

impl SubFilter {
    /// Constant velocity state prediction.
    pub fn predict_cv(&mut self, dt: f64, q: f64) {
        for i in 0..3 {
            self.state[i] += self.state[i + 3] * dt;
        }
        let dt2 = dt * dt;
        let dt3 = dt2 * dt / 3.0;
        let dt4 = dt2 * dt2 / 4.0;
        for i in 0..3 {
            self.cov[i][i] += dt4 * q;
            self.cov[i + 3][i + 3] += dt2 * q;
            self.cov[i][i + 3] += dt3 * q;
            self.cov[i + 3][i] += dt3 * q;
        }
    }
}

impl SubFilter {
    /// Constant acceleration state prediction.
    pub fn predict_ca(&mut self, dt: f64, q: f64) {
        for i in 0..3 {
            self.state[i] += self.state[i + 3] * dt;
        }
        let dt2 = dt * dt;
        let dt3 = dt2 * dt / 2.0;
        let dt4 = dt2 * dt2 / 2.0;
        for i in 0..3 {
            self.cov[i][i] += dt4 * q;
            self.cov[i + 3][i + 3] += dt2 * q;
            self.cov[i][i + 3] += dt3 * q;
            self.cov[i + 3][i] += dt3 * q;
        }
    }
}

impl SubFilter {
    /// Coordinated turn state prediction.
    pub fn predict_ct(&mut self, dt: f64, omega: f64, q: f64) {
        let (sin_w, cos_w) = if omega.abs() > 1e-4 {
            let w_dt = omega * dt;
            (w_dt.sin() / omega, (1.0 - w_dt.cos()) / omega)
        } else {
            (dt, 0.0)
        };
        let vx = self.state[3];
        let vy = self.state[4];
        self.state[0] += sin_w * vx - cos_w * vy;
        self.state[1] += cos_w * vx + sin_w * vy;
        self.state[2] += self.state[5] * dt;
        self.state[3] = (omega * dt).cos() * vx - (omega * dt).sin() * vy;
        self.state[4] = (omega * dt).sin() * vx + (omega * dt).cos() * vy;
        for i in 0..6 {
            self.cov[i][i] += q * dt;
        }
    }
}

impl SubFilter {
    /// Measurement update with 3D position.
    pub fn update_pos(&mut self, z: [f64; 3], r: f64) -> f64 {
        let mut likelihood = 1.0;
        for i in 0..3 {
            let y = z[i] - self.state[i];
            let s = self.cov[i][i] + r;
            let k = self.cov[i][i] / s;
            self.state[i] += k * y;
            self.state[i + 3] += (self.cov[i + 3][i] / s) * y;
            self.cov[i][i] *= 1.0 - k;
            self.cov[i + 3][i + 3] -= (self.cov[i + 3][i] * self.cov[i + 3][i]) / s;
            let exponent = -0.5 * (y * y) / s;
            let norm = 1.0 / (core::f64::consts::TAU * s).sqrt();
            likelihood *= norm * exponent.exp();
        }
        likelihood.max(1e-12)
    }
}
