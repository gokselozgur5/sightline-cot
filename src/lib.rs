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
pub mod cot_beacon;
pub use cot_beacon::{CotBeaconError, EmergencyBeacon, EmergencySeverity, EmergencyType};
pub mod cot_detail;
pub mod cot_sensor;
pub mod cot_sensor_fov;
pub use cot_sensor_fov::{
    compute_ground_footprint, is_target_in_sensor_fov, SensorCone, SensorFootprint, SensorFovError,
};
pub mod cot_shape;
pub use cot_shape::{CotShape, CotShapeError, CotShapeType, CotVertex};
pub mod cot_task;
pub use cot_task::{CotTaskError, TacticalTask, TaskAction, TaskPoint, TaskPriority, TaskStatus};
pub mod cot_circle;
pub use cot_circle::{CotCircle, CotCircleError, CotRangeRings};
pub mod sensor_frustum;
pub use sensor_frustum::{
    compute_frustum_ground_enu, compute_frustum_rays_enu, generate_sensor_footprint,
    point_in_polygon_enu, polygon_area_enu, polygon_perimeter_enu, FrustumError, FrustumFootprint,
    PlatformPose, SensorFrustumConfig, SensorGimbal,
};
pub mod dem_bilinear;
pub mod egm96_geoid;
pub mod elevation_mask;
pub use egm96_geoid::{
    normalize_lon_180, normalize_lon_360, sample_egm96_undulation, Egm96Error, Egm96Geoid,
};
pub use elevation_mask::{
    ElevationMaskError, ElevationMaskValidator, MaskSector, MaskingEvaluation, MaskingStatus,
    ObstaclePoint, ObstacleProfile,
};
pub mod ffi;
pub mod geo_helmert;
pub mod geo_molodensky;
pub mod geo_velocity;
pub mod geo_vincenty;
pub mod geo_vincenty_direct;
pub use geo_vincenty::{
    vincenty_direct, vincenty_direct_ellipsoid, vincenty_geodesic_path,
    vincenty_intermediate_points, vincenty_inverse, vincenty_inverse_ellipsoid, VincentyDirect,
    VincentyError, VincentyInverse,
};
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
pub mod mgrs_parse;
pub mod mgrs_truncate;
pub use mgrs::{
    format_mgrs, lat_to_mgrs_band, lon_to_utm_zone, mgrs_10figure_to_utm, mgrs_10figure_to_wgs84,
    parse_mgrs_10figure, utm_to_mgrs_10figure, utm_to_wgs84, wgs84_to_mgrs_10figure, wgs84_to_utm,
    GridError, Mgrs10Figure, MgrsString, UtmCoord,
};
pub use mgrs_parse::{
    parse_mgrs, parse_mgrs_to_utm, parse_mgrs_to_wgs84, MgrsParseError, ParsedMgrs,
};
pub mod mil_std;
pub mod proto;
pub mod rhumb_line;
pub use rhumb_line as geo_rhumb;
pub use rhumb_line::{
    great_circle_distance, interpolate_by_count, interpolate_by_step, rhumb_bearing,
    rhumb_destination, rhumb_destination_radius, rhumb_distance, rhumb_distance_and_bearing,
    rhumb_distance_radius, rhumb_intermediate, rhumb_intermediate_fraction, RhumbLineError,
    RhumbPath, RhumbSegmentComparison, RhumbWaypoint,
};
pub mod sidc;
pub mod stanag4609;
pub use mil_std::{
    parse_mil_std, MilStdAffiliation, MilStdDimension, MilStdEchelon, MilStdError, MilStdStatus,
    MilStdSymbol,
};
pub use sidc::{SidcAffiliation, SidcDomain, TacticalSymbol};
pub use stanag4609::{StanagError, StanagTimestamp};
pub mod geo_angle;
pub mod geo_area;
pub mod geo_polygon_area;
pub use geo_polygon_area::{
    compute_cot_polygon_geometry, geodesic_polygon_area, is_counter_clockwise,
    polygon_geodesic_perimeter, GeodesicAreaError, GeodesicPolygonArea,
};
pub mod geo_bbox;
pub mod geo_centroid;
pub mod geo_compass;
pub mod geo_crosstrack;
pub use geo_crosstrack as geo_cross_track;
pub use geo_crosstrack::{
    along_track_distance_m, cross_track_distance, cross_track_distance_m, cross_track_error,
    CrossTrackResult,
};
pub mod geo_aer;
pub mod geo_ddm;
pub mod geo_destination;
pub mod geo_dms;
pub mod geo_enu;
pub use geo_aer::{
    aer_to_ecef, aer_to_geodetic, aer_to_sez, ecef_to_aer, ecef_to_sez, enu_to_sez,
    geodetic_to_aer, sez_to_aer, sez_to_ecef, sez_to_enu, AerCoord, AerError, SezCoord,
};
pub mod geo_ecef;
pub mod geo_frame;
pub use geo_frame::{GeoFrameError, LocalFrame};
pub mod geo_great_circle;
pub mod geo_greatcircle;
pub use geo_great_circle::{GreatCircleError, GreatCirclePath, Waypoint};
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
pub mod geo_perimeter;
pub mod geo_polyline;
pub mod geo_polyline_nearest;
pub mod geo_radii;
pub mod geo_simplify;
pub mod geoid_loader;
pub use geo_simplify::{simplify_polyline, simplify_polyline_in_frame, PlanarFrame};
pub mod cam_distorted_pinhole;
pub mod geo_sphere;
pub mod tracker;
pub mod tracker_cov_adaptive;
pub use tracker_cov_adaptive::{
    compute_adaptive_scale, scale_measurement_variance, AdaptiveCovarianceConfig,
    AdaptiveCovarianceError,
};
pub mod utm_forced;
pub mod utm_mgrs;
pub mod utm_string;
pub use utm_forced::{
    is_adjacent_zone, utm_forced_to_wgs84, wgs84_to_utm_forced, zone_distance_delta,
};
pub mod geo_unified;
pub use cam_distorted_pinhole::DistortedPinhole;
pub use geo_unified::{
    is_polar_latitude, is_utm_latitude, unified_to_wgs84, wgs84_to_unified, GridSystem,
    UnifiedCoord,
};
pub mod geoid;
pub use geoid::{
    ellipsoidal_to_orthometric, orthometric_to_ellipsoidal, sample_geoid_bilinear, ConstantGeoid,
    Geoid, GeoidBilinearGrid, GeoidError, GeoidGrid, GeoidModel,
};
pub mod ellipsoid;
pub use ellipsoid::{Ellipsoid, GRS80, WGS72, WGS84};
pub mod geohash_neighbors;
pub use geohash_neighbors::{
    geohash_neighbor, geohash_neighbors, GeohashDirection, GeohashNeighbors,
};
pub mod geo_los;
pub mod geo_ups;
pub mod geohash;
pub use geo_los::{
    compute_geo_los, elevation_angle_rad, has_line_of_sight, horizon_distance_m,
    slant_range_geodetic, GeoLosError, GeoPoint3D, LosGeometry,
};
pub use geo_unified::{ups_to_wgs84, wgs84_to_ups, UpsCoord};
pub use geohash::{decode_geohash, encode_geohash, GeohashError};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo_los_crate_export() {
        let obs = GeoPoint3D::new(0.0, 0.0, 0.0).unwrap();
        let tgt = GeoPoint3D::new(0.0, 0.0, 100.0).unwrap();
        let geom = compute_geo_los(&obs, &tgt).unwrap();
        assert!((geom.slant_range_m - 100.0).abs() < 1e-3);
    }
    #[test]
    fn test_cot_sensor_fov_crate_export() {
        let cone = SensorCone::new(0.0, -45.0, 30.0, 1000.0).unwrap();
        assert!(cone.contains_relative_point([0.0, 100.0, -100.0]));
    }
    #[test]
    fn test_egm96_geoid_crate_export() {
        let geoid = Egm96Geoid::coarse_reference_grid();
        assert!((geoid.get_node(0, 0) - (-30.0)).abs() < 1e-6);
    }
    #[test]
    fn test_cot_task_crate_export() {
        let task = TacticalTask::new("EXPORT-TEST");
        assert_eq!(task.name, "EXPORT-TEST");
    }
    #[test]
    fn test_sensor_frustum_crate_export() {
        let p = PlatformPose::new(0.0, 0.0, 500.0, 0.0, 0.0, 0.0).unwrap();
        let g = SensorGimbal::new(0.0, -90.0, 0.0).unwrap();
        let c = SensorFrustumConfig::new(45.0, 45.0, 2000.0, 1.0).unwrap();
        let fp = generate_sensor_footprint(&p, &g, &c, 0.0).unwrap();
        assert_eq!(fp.vertices_enu.len(), 4);
    }
    #[test]
    fn test_spatial_rtree_crate_export() {
        let mut tree = crate::SpatialRTree::new();
        tree.insert(crate::SpatialEnvelope::from_point(0.0, 0.0), 42);
        assert_eq!(tree.len(), 1);
    }
    #[test]
    fn sanity() {
        assert_eq!(VERSION, "0.1.0");
    }
}
pub mod track_assoc;
pub use track_assoc::{
    compute_spatial_matrix_2d, compute_spatial_matrix_3d, euclidean_dist_2d, euclidean_dist_3d,
    solve_nearest_neighbor, solve_optimal_hungarian, AssocConfig, AssocError, AssocMatch,
    AssocResult, DistanceMatrix,
};

pub mod xml_canonicalizer;
pub use xml_canonicalizer::{
    canonicalize_cot_xml, validate_cot_xml, CanonicalizeOptions, CotSchemaValidator,
    CotValidationError, CotXmlCanonicalizer,
};
pub mod spatial_rtree;
pub use spatial_rtree::{normalize_lon, RTreeEntry, SpatialEnvelope, SpatialRTree};

pub mod sidc_parser;
pub use sidc_parser::{
    cot_type_to_sidc20, generate_sidc2525d, parse_sidc2525d, sidc20_to_cot_type, Echelon,
    MilStd2525DSymbol, SidcParseError, StandardIdentity, SymbolSet, SymbolStatus,
};

pub mod terrain_sampler;
pub use terrain_sampler::{
    cot_target_from_intercept, GeoTiffElevationGrid, InterpolationMethod, TerrainIntercept,
    TerrainInterceptSolver, TerrainRay, TerrainSamplerError,
};

pub mod kalman_target;
pub use kalman_target::{
    KalmanTargetError, KinematicState, TacticalTargetFilter, TargetFilterConfig,
};

pub mod sax_parser;
pub use sax_parser::{
    CotChunkStreamer, CotEventView, CotPointView, CotSaxParser, SaxAttribute, SaxAttributes,
    SaxAttributesIter, SaxError, SaxEvent, SaxReader,
};
