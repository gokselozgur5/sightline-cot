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
