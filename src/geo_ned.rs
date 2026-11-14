//! Local ENU (East-North-Up) and NED (North-East-Down) frame conversions.

pub use crate::geo_enu::EnuError;

/// Rotate a 3D vector from ENU (East, North, Up) to NED (North, East, Down).
#[inline]
pub fn enu_to_ned(enu: [f64; 3]) -> [f64; 3] {
    [enu[1], enu[0], -enu[2]]
}
