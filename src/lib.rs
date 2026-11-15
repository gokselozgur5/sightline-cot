//! Sightline CoT core library

pub mod cam_project;
pub mod cam_slant_range;
pub use cam_slant_range::{
    compute_slant_range_and_grazing, slant_range_for_pixel, slant_range_from_pose_and_ground,
    SlantRangeGeometry,
};
pub mod cam_frustum;
pub mod camera;
pub mod classifier;
pub mod cot;
pub mod cot_detail;
pub mod cot_sensor;
pub mod dem_bilinear;
pub mod ffi;
pub mod geo_helmert;
pub mod geo_molodensky;
pub mod geo_velocity;
pub mod geo_vincenty;
pub mod geo_vincenty_direct;
pub mod geofence;
pub use geofence::{Geofence, GeofenceBreach, GeofenceState};
pub mod imm;
pub mod imm_transition;
pub mod los;
pub mod los_ground;
pub mod maidenhead;
pub mod mgrs;
pub mod mgrs_decode;
pub mod mgrs_encode;
pub mod mgrs_truncate;
pub use mgrs::{
    format_mgrs, lat_to_mgrs_band, lon_to_utm_zone, utm_to_wgs84, wgs84_to_utm, GridError,
    MgrsString, UtmCoord,
};
pub mod proto;
pub mod sidc;
pub use sidc::{SidcAffiliation, SidcDomain, TacticalSymbol};
pub mod geo_angle;
pub mod geo_area;
pub mod geo_bbox;
pub mod geo_centroid;
pub mod geo_compass;
pub mod geo_cross_track;
pub mod geo_crosstrack;
pub use geo_crosstrack::{
    along_track_distance_m, cross_track_distance_m, cross_track_error, CrossTrackResult,
};
pub mod geo_ddm;
pub mod geo_dms;
pub mod geo_destination;
pub mod geo_enu;
pub mod geo_frame;
pub mod geo_greatcircle;
pub mod geo_haversine;
pub use geo_haversine::{
    archav, hav, haversine_bearing_deg, haversine_bearing_rad, haversine_distance_m,
    haversine_distance_with_radius,
};
pub mod geo_hull;
pub mod geo_interp;
pub mod geo_intersect;
pub mod geo_meridian_arc;
pub mod geo_midpoint;
pub mod geo_ned;
pub mod geoid_loader;
pub mod geo_perimeter;
pub mod geo_polyline;
pub mod geo_polyline_nearest;
pub mod geo_radii;
pub mod geo_rhumb;
pub mod geo_simplify;
pub use geo_simplify::{simplify_polyline, LocalFrame};
pub mod cam_distorted_pinhole;
pub mod geo_sphere;
pub mod tracker;
pub mod tracker_cov_adaptive;
pub use tracker_cov_adaptive::{
    compute_adaptive_scale, scale_measurement_variance, AdaptiveCovarianceConfig,
    AdaptiveCovarianceError,
};
pub mod utm_mgrs;
pub mod utm_string;
pub use cam_distorted_pinhole::DistortedPinhole;
pub mod geoid;
pub use geoid::{
    ellipsoidal_to_orthometric, orthometric_to_ellipsoidal, sample_geoid_bilinear, ConstantGeoid,
    Geoid, GeoidBilinearGrid, GeoidError, GeoidGrid, GeoidModel,
};
pub mod ellipsoid;
pub use ellipsoid::{Ellipsoid, GRS80, WGS72, WGS84};

pub const VERSION: &str = "0.1.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        assert_eq!(VERSION, "0.1.0");
    }
}
