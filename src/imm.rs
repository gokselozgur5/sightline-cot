#![allow(clippy::approx_constant, clippy::needless_range_loop)]
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
            let (p0, p1, p2) = (self.cov[i][i], self.cov[i][i + 3], self.cov[i + 3][i + 3]);
            self.cov[i][i] = p0 + 2.0 * dt * p1 + dt * dt * p2;
            self.cov[i][i + 3] = p1 + dt * p2;
            self.cov[i + 3][i] = self.cov[i][i + 3];
            let dt2 = dt * dt;
            self.cov[i][i] += 0.25 * dt2 * dt2 * q;
            self.cov[i + 3][i + 3] += dt2 * q;
            self.cov[i][i + 3] += 0.5 * dt2 * dt * q;
            self.cov[i + 3][i] = self.cov[i][i + 3];
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

/// Interacting Multiple Model (IMM) multi-model filter.
#[derive(Debug, Clone, PartialEq)]
pub struct ImmFilter {
    pub filters: [SubFilter; NUM_MODELS],
    pub probs: [f64; NUM_MODELS],
    pub trans_mat: [[f64; NUM_MODELS]; NUM_MODELS],
    pub config: ImmConfig,
}

impl ImmFilter {
    pub fn new(init_pos: [f64; 3], config: ImmConfig) -> Self {
        let pos_var = config.pos_noise_m * config.pos_noise_m;
        let vel_var = 10.0;
        let filters = [
            SubFilter::new(init_pos, pos_var, vel_var),
            SubFilter::new(init_pos, pos_var, vel_var),
            SubFilter::new(init_pos, pos_var, vel_var),
        ];
        let probs = [0.8, 0.1, 0.1];
        let trans_mat = [[0.85, 0.10, 0.05], [0.15, 0.75, 0.10], [0.10, 0.15, 0.75]];
        Self {
            filters,
            probs,
            trans_mat,
            config,
        }
    }
}

impl ImmFilter {
    /// Compute normalization factors for model mixing.
    pub fn compute_c_bar(&self) -> [f64; NUM_MODELS] {
        let mut c_bar = [0.0; NUM_MODELS];
        for j in 0..NUM_MODELS {
            for i in 0..NUM_MODELS {
                c_bar[j] += self.trans_mat[i][j] * self.probs[i];
            }
        }
        c_bar
    }
}

impl ImmFilter {
    /// Model mixing step prior to prediction.
    pub fn mix(&mut self) {
        let c_bar = self.compute_c_bar();
        let mut mu = [[0.0; NUM_MODELS]; NUM_MODELS];
        for i in 0..NUM_MODELS {
            for j in 0..NUM_MODELS {
                let p = self.trans_mat[i][j] * self.probs[i];
                mu[i][j] = if c_bar[j] > 1e-12 {
                    p / c_bar[j]
                } else {
                    1.0 / 3.0
                };
            }
        }
        let old = self.filters.clone();
        for j in 0..NUM_MODELS {
            let mut st = [0.0; 6];
            for i in 0..NUM_MODELS {
                for k in 0..6 {
                    st[k] += mu[i][j] * old[i].state[k];
                }
            }
            self.filters[j].state = st;
        }
    }
}

impl ImmFilter {
    /// Predict all sub-filters forward by dt.
    pub fn predict(&mut self, dt: f64) {
        self.filters[MODEL_CV].predict_cv(dt, self.config.cv_proc_noise);
        self.filters[MODEL_CA].predict_ca(dt, self.config.ca_proc_noise);
        self.filters[MODEL_CT].predict_ct(
            dt,
            self.config.turn_rate_rad_s,
            self.config.ct_proc_noise,
        );
    }
}

impl ImmFilter {
    /// Update each model with observation and update probabilities.
    pub fn update(&mut self, z: [f64; 3]) {
        let r = self.config.pos_noise_m * self.config.pos_noise_m;
        let mut lik = [0.0; NUM_MODELS];
        for j in 0..NUM_MODELS {
            lik[j] = self.filters[j].update_pos(z, r);
        }
        let mut sum_p = 0.0;
        for j in 0..NUM_MODELS {
            self.probs[j] *= lik[j];
            sum_p += self.probs[j];
        }
        if sum_p > 1e-12 {
            for j in 0..NUM_MODELS {
                self.probs[j] /= sum_p;
            }
        } else {
            self.probs = [1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0];
        }
    }
}

impl ImmFilter {
    /// Step full IMM cycle: mix -> predict -> update.
    pub fn step(&mut self, dt: f64, z: [f64; 3]) -> [f64; 6] {
        self.mix();
        self.predict(dt);
        self.update(z);
        self.fused_state()
    }
    /// Compute probability-weighted fused state output.
    pub fn fused_state(&self) -> [f64; 6] {
        let mut fused = [0.0; 6];
        for j in 0..NUM_MODELS {
            for k in 0..6 {
                fused[k] += self.probs[j] * self.filters[j].state[k];
            }
        }
        fused
    }
}

impl ImmFilter {
    /// Returns true if a high-G turn or acceleration maneuver is active.
    pub fn is_maneuvering(&self) -> bool {
        self.probs[MODEL_CA] + self.probs[MODEL_CT] > 0.5
    }
}

#[cfg(test)]
mod imm_tests {
    use super::*;

    #[test]
    fn test_imm_constant_velocity_tracking() {
        let cfg = ImmConfig::default();
        let mut imm = ImmFilter::new([0.0, 0.0, 0.0], cfg);
        for step in 1..=50 {
            let t = step as f64 * 0.1;
            let z = [10.0 * t, 0.0, 0.0];
            imm.step(0.1, z);
        }
        assert!(imm.probs[MODEL_CV] > 0.4);
        let st = imm.fused_state();
        assert!((st[3] - 10.0).abs() < 1.0);
    }
}

#[cfg(test)]
mod imm_turn_tests {
    use super::*;

    #[test]
    fn test_imm_coordinated_turn_maneuver_detection() {
        let cfg = ImmConfig::default();
        let mut imm = ImmFilter::new([0.0, 0.0, 0.0], cfg);
        let w = 0.5235987756; // 30 deg/s
        for step in 1..=30 {
            let t = step as f64 * 0.1;
            let z = [(w * t).sin() * 50.0, (1.0 - (w * t).cos()) * 50.0, 0.0];
            imm.step(0.1, z);
        }
        assert!(imm.is_maneuvering() || imm.probs[MODEL_CT] > imm.probs[MODEL_CV]);
    }
}

#[cfg(test)]
mod imm_ca_tests {
    use super::*;

    #[test]
    fn test_imm_high_g_acceleration_detection() {
        let cfg = ImmConfig::default();
        let mut imm = ImmFilter::new([0.0, 0.0, 0.0], cfg);
        let a = 20.0;
        for step in 1..=30 {
            let t = step as f64 * 0.1;
            let z = [0.5 * a * t * t, 0.0, 0.0];
            imm.step(0.1, z);
        }
        assert!(imm.is_maneuvering());
    }
}
