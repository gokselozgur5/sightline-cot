//! Sightline CoT core library

pub mod camera;
pub mod classifier;
pub mod cot;
pub mod ffi;
pub mod geo_helmert;
pub mod geo_molodensky;
pub mod geo_velocity;
pub mod geo_vincenty;
pub mod geofence;
pub use geofence::{Geofence, GeofenceBreach, GeofenceState};
pub mod imm;
pub mod los;
pub mod maidenhead;
pub mod mgrs;
pub mod mgrs_encode;
pub use mgrs::{
    format_mgrs, lat_to_mgrs_band, lon_to_utm_zone, utm_to_wgs84, wgs84_to_utm, GridError,
    MgrsString, UtmCoord,
};
pub mod proto;
pub mod sidc;
pub use sidc::{SidcAffiliation, SidcDomain, TacticalSymbol};
pub mod geo_angle;
pub mod geo_area;
pub mod geo_centroid;
pub mod geo_compass;
pub mod geo_cross_track;
pub mod geo_ddm;
pub mod geo_destination;
pub mod geo_greatcircle;
pub mod geo_haversine;
pub use geo_haversine::{
    archav, hav, haversine_bearing_deg, haversine_bearing_rad, haversine_distance_m,
    haversine_distance_with_radius,
};
pub mod geo_hull;
pub mod geo_meridian_arc;
pub mod geo_perimeter;
pub mod geo_polyline;
pub mod geo_polyline_nearest;
pub mod geo_radii;
pub mod geo_rhumb;
pub mod geo_simplify;
pub use geo_simplify::{simplify_polyline, LocalFrame};
pub mod tracker;
pub mod utm_mgrs;
pub mod utm_string;

pub const VERSION: &str = "0.1.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        assert_eq!(VERSION, "0.1.0");
    }
}
