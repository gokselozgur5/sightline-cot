//! Sightline CoT core library

pub mod camera;
pub mod classifier;
pub mod cot;
pub use cot::CotTcpStreamer;
pub mod ffi;
pub mod geofence;
pub use geofence::{Geofence, GeofenceBreach, GeofenceStatus};
pub mod tracker;

pub const VERSION: &str = "0.1.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        assert_eq!(VERSION, "0.1.0");
    }
}
