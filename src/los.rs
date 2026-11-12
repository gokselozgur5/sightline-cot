//! 3D Line-of-Sight (LOS) terrain masking and horizon analysis.

use crate::camera::DemGrid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position3D {
    pub east_m: f64,
    pub north_m: f64,
    pub up_m: f64,
}
