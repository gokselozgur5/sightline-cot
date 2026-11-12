//! Interacting Multiple Model (IMM-EKF) Multi-Model Target Tracker.

/// Number of parallel kinematic models in IMM filter.
pub const NUM_MODELS: usize = 3;
/// Constant Velocity (CV) model index.
pub const MODEL_CV: usize = 0;
/// Constant Acceleration (CA) model index.
pub const MODEL_CA: usize = 1;
/// Coordinated Turn (CT) model index.
pub const MODEL_CT: usize = 2;
