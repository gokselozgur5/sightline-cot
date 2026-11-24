//! Sightline CoT core library

#[cfg(feature = "experimental")]
pub mod cam_project;
#[cfg(feature = "experimental")]
pub mod cam_slant_range;
#[cfg(feature = "experimental")]
pub use cam_slant_range::{
    compute_slant_range_and_grazing, slant_range_for_pixel, slant_range_from_pose_and_ground,
    SlantRangeGeometry,
};
#[cfg(feature = "experimental")]
pub mod cam_frustum;
pub mod camera;
pub mod classifier;
pub mod cot;
#[cfg(feature = "experimental")]
pub mod cot_beacon;
#[cfg(feature = "experimental")]
pub use cot_beacon::{CotBeaconError, EmergencyBeacon, EmergencySeverity, EmergencyType};
#[cfg(feature = "experimental")]
pub mod cot_detail;
#[cfg(feature = "experimental")]
pub mod cot_sensor;
#[cfg(feature = "experimental")]
pub mod cot_sensor_fov;
#[cfg(feature = "experimental")]
pub use cot_sensor_fov::{
    compute_ground_footprint, is_target_in_sensor_fov, SensorCone, SensorFootprint, SensorFovError,
};
#[cfg(feature = "experimental")]
pub mod cot_shape;
#[cfg(feature = "experimental")]
pub use cot_shape::{CotShape, CotShapeError, CotShapeType, CotVertex};
#[cfg(feature = "experimental")]
pub mod cot_task;
#[cfg(feature = "experimental")]
pub use cot_task::{CotTaskError, TacticalTask, TaskAction, TaskPoint, TaskPriority, TaskStatus};
#[cfg(feature = "experimental")]
pub mod cot_circle;
#[cfg(feature = "experimental")]
pub use cot_circle::{CotCircle, CotCircleError, CotRangeRings};
#[cfg(feature = "experimental")]
pub mod sensor_frustum;
#[cfg(feature = "experimental")]
pub use sensor_frustum::{
    compute_frustum_ground_enu, compute_frustum_rays_enu, generate_sensor_footprint,
    point_in_polygon_enu, polygon_area_enu, polygon_perimeter_enu, FrustumError, FrustumFootprint,
    PlatformPose, SensorFrustumConfig, SensorGimbal,
};
pub mod dem_bilinear;
#[cfg(feature = "geodesy")]
pub mod egm96_geoid;
#[cfg(feature = "experimental")]
pub mod elevation_mask;
#[cfg(feature = "geodesy")]
pub use egm96_geoid::{
    normalize_lon_180, normalize_lon_360, sample_egm96_undulation, Egm96Error, Egm96Geoid,
};
#[cfg(feature = "experimental")]
pub use elevation_mask::{
    ElevationMaskError, ElevationMaskValidator, MaskSector, MaskingEvaluation, MaskingStatus,
    ObstaclePoint, ObstacleProfile,
};
pub mod ffi;
#[cfg(feature = "geodesy")]
pub mod geo_helmert;
#[cfg(feature = "geodesy")]
pub mod geo_molodensky;
#[cfg(feature = "geodesy")]
pub mod geo_velocity;
#[cfg(feature = "geodesy")]
pub mod geo_vincenty;
#[cfg(feature = "geodesy")]
pub mod geo_vincenty_direct;
#[cfg(feature = "geodesy")]
pub use geo_vincenty::{
    vincenty_direct, vincenty_direct_ellipsoid, vincenty_geodesic_path,
    vincenty_intermediate_points, vincenty_inverse, vincenty_inverse_ellipsoid, VincentyDirect,
    VincentyError, VincentyInverse,
};
#[cfg(feature = "experimental")]
pub mod geofence;
#[cfg(feature = "experimental")]
pub use geofence::{Geofence, GeofenceBreach, GeofenceState};
#[cfg(feature = "experimental")]
pub mod imm;
#[cfg(feature = "experimental")]
pub mod imm_transition;
#[cfg(feature = "experimental")]
pub mod los;
#[cfg(feature = "experimental")]
pub mod los_ground;
#[cfg(feature = "grid")]
pub mod maidenhead;
#[cfg(feature = "grid")]
pub mod mgrs;
#[cfg(feature = "grid")]
pub mod mgrs_decode;
#[cfg(feature = "grid")]
pub mod mgrs_encode;
#[cfg(feature = "grid")]
pub mod mgrs_parse;
#[cfg(feature = "grid")]
pub mod mgrs_truncate;
#[cfg(feature = "grid")]
pub use mgrs::{
    format_mgrs, lat_to_mgrs_band, lon_to_utm_zone, mgrs_10figure_to_utm, mgrs_10figure_to_wgs84,
    parse_mgrs_10figure, utm_to_mgrs_10figure, utm_to_wgs84, wgs84_to_mgrs_10figure, wgs84_to_utm,
    GridError, Mgrs10Figure, MgrsString, UtmCoord,
};
#[cfg(feature = "grid")]
pub use mgrs_parse::{
    parse_mgrs, parse_mgrs_to_utm, parse_mgrs_to_wgs84, MgrsParseError, ParsedMgrs,
};
pub mod mil_std;
pub mod proto;
#[cfg(feature = "geodesy")]
pub mod rhumb_line;
#[cfg(feature = "geodesy")]
pub use rhumb_line as geo_rhumb;
#[cfg(feature = "geodesy")]
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
#[cfg(feature = "geodesy")]
pub mod geo_angle;
#[cfg(feature = "geodesy")]
pub mod geo_area;
#[cfg(feature = "geodesy")]
pub mod geo_polygon_area;
#[cfg(feature = "geodesy")]
pub use geo_polygon_area::{
    compute_cot_polygon_geometry, geodesic_polygon_area, is_counter_clockwise,
    polygon_geodesic_perimeter, GeodesicAreaError, GeodesicPolygonArea,
};
#[cfg(feature = "geodesy")]
pub mod geo_bbox;
#[cfg(feature = "geodesy")]
pub mod geo_centroid;
#[cfg(feature = "geodesy")]
pub mod geo_compass;
#[cfg(feature = "geodesy")]
pub mod geo_crosstrack;
#[cfg(feature = "geodesy")]
pub use geo_crosstrack as geo_cross_track;
#[cfg(feature = "geodesy")]
pub use geo_crosstrack::{
    along_track_distance_m, cross_track_distance, cross_track_distance_m, cross_track_error,
    CrossTrackResult,
};
#[cfg(feature = "geodesy")]
pub mod geo_aer;
#[cfg(feature = "geodesy")]
pub mod geo_ddm;
#[cfg(feature = "geodesy")]
pub mod geo_destination;
#[cfg(feature = "geodesy")]
pub mod geo_dms;
#[cfg(feature = "geodesy")]
pub mod geo_enu;
#[cfg(feature = "geodesy")]
pub use geo_aer::{
    aer_to_ecef, aer_to_geodetic, aer_to_sez, ecef_to_aer, ecef_to_sez, enu_to_sez,
    geodetic_to_aer, sez_to_aer, sez_to_ecef, sez_to_enu, AerCoord, AerError, SezCoord,
};
pub mod geo_ecef;
#[cfg(feature = "geodesy")]
pub mod geo_frame;
#[cfg(feature = "geodesy")]
pub use geo_frame::{GeoFrameError, LocalFrame};
#[cfg(feature = "geodesy")]
pub mod geo_great_circle;
#[cfg(feature = "geodesy")]
pub mod geo_greatcircle;
#[cfg(feature = "geodesy")]
pub use geo_great_circle::{GreatCircleError, GreatCirclePath, Waypoint};
#[cfg(feature = "geodesy")]
pub mod geo_haversine;
#[cfg(feature = "geodesy")]
pub use geo_haversine::{
    archav, hav, haversine_bearing_deg, haversine_bearing_rad, haversine_distance_m,
    haversine_distance_with_radius,
};
#[cfg(feature = "geodesy")]
pub mod geo_hull;
#[cfg(feature = "geodesy")]
pub mod geo_interp;
#[cfg(feature = "geodesy")]
pub mod geo_intersect;
#[cfg(feature = "geodesy")]
pub mod geo_meridian_arc;
#[cfg(feature = "geodesy")]
pub mod geo_midpoint;
#[cfg(feature = "geodesy")]
pub mod geo_ned;
#[cfg(feature = "geodesy")]
pub mod geo_perimeter;
#[cfg(feature = "geodesy")]
pub mod geo_polyline;
#[cfg(feature = "geodesy")]
pub mod geo_polyline_nearest;
#[cfg(feature = "geodesy")]
pub mod geo_radii;
#[cfg(feature = "geodesy")]
pub mod geo_simplify;
#[cfg(feature = "geodesy")]
pub mod geoid_loader;
#[cfg(feature = "geodesy")]
pub use geo_simplify::{simplify_polyline, simplify_polyline_in_frame, PlanarFrame};
#[cfg(feature = "experimental")]
pub mod cam_distorted_pinhole;
#[cfg(feature = "geodesy")]
pub mod geo_sphere;
pub mod tracker;
pub mod tracker_cov_adaptive;
pub use tracker_cov_adaptive::{
    compute_adaptive_scale, scale_measurement_variance, AdaptiveCovarianceConfig,
    AdaptiveCovarianceError,
};
#[cfg(feature = "grid")]
pub mod utm_forced;
#[cfg(feature = "grid")]
pub mod utm_mgrs;
#[cfg(feature = "grid")]
pub mod utm_string;
#[cfg(feature = "grid")]
pub use utm_forced::{
    is_adjacent_zone, utm_forced_to_wgs84, wgs84_to_utm_forced, zone_distance_delta,
};
#[cfg(feature = "grid")]
pub mod geo_unified;
#[cfg(feature = "experimental")]
pub use cam_distorted_pinhole::DistortedPinhole;
#[cfg(feature = "grid")]
pub use geo_unified::{
    is_polar_latitude, is_utm_latitude, unified_to_wgs84, wgs84_to_unified, GridSystem,
    UnifiedCoord,
};
#[cfg(feature = "geodesy")]
pub mod geoid;
#[cfg(feature = "geodesy")]
pub use geoid::{
    ellipsoidal_to_orthometric, orthometric_to_ellipsoidal, sample_geoid_bilinear, ConstantGeoid,
    Geoid, GeoidBilinearGrid, GeoidError, GeoidGrid, GeoidModel,
};
pub mod ellipsoid;
pub use ellipsoid::{Ellipsoid, GRS80, WGS72, WGS84};
#[cfg(feature = "grid")]
pub mod geohash_neighbors;
#[cfg(feature = "grid")]
pub use geohash_neighbors::{
    geohash_neighbor, geohash_neighbors, GeohashDirection, GeohashNeighbors,
};
#[cfg(feature = "geodesy")]
pub mod geo_los;
#[cfg(feature = "grid")]
pub mod geo_ups;
#[cfg(feature = "grid")]
pub mod geohash;
#[cfg(feature = "geodesy")]
pub use geo_los::{
    compute_geo_los, elevation_angle_rad, has_line_of_sight, horizon_distance_m,
    slant_range_geodetic, GeoLosError, GeoPoint3D, LosGeometry,
};
#[cfg(feature = "grid")]
pub use geo_unified::{ups_to_wgs84, wgs84_to_ups, UpsCoord};
#[cfg(feature = "grid")]
pub use geohash::{decode_geohash, encode_geohash, GeohashError};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "geodesy")]
    fn test_geo_los_crate_export() {
        let obs = GeoPoint3D::new(0.0, 0.0, 0.0).unwrap();
        let tgt = GeoPoint3D::new(0.0, 0.0, 100.0).unwrap();
        let geom = compute_geo_los(&obs, &tgt).unwrap();
        assert!((geom.slant_range_m - 100.0).abs() < 1e-3);
    }
    #[test]
    #[cfg(feature = "experimental")]
    fn test_cot_sensor_fov_crate_export() {
        let cone = SensorCone::new(0.0, -45.0, 30.0, 1000.0).unwrap();
        assert!(cone.contains_relative_point([0.0, 100.0, -100.0]));
    }
    #[test]
    #[cfg(feature = "geodesy")]
    fn test_egm96_geoid_crate_export() {
        let geoid = Egm96Geoid::coarse_reference_grid();
        assert!((geoid.get_node(0, 0) - (-29.53)).abs() < 1e-6);
    }
    #[test]
    #[cfg(feature = "experimental")]
    fn test_cot_task_crate_export() {
        let task = TacticalTask::new("EXPORT-TEST");
        assert_eq!(task.name, "EXPORT-TEST");
    }
    #[test]
    #[cfg(feature = "experimental")]
    fn test_sensor_frustum_crate_export() {
        let p = PlatformPose::new(0.0, 0.0, 500.0, 0.0, 0.0, 0.0).unwrap();
        let g = SensorGimbal::new(0.0, -90.0, 0.0).unwrap();
        let c = SensorFrustumConfig::new(45.0, 45.0, 2000.0, 1.0).unwrap();
        let fp = generate_sensor_footprint(&p, &g, &c, 0.0).unwrap();
        assert_eq!(fp.vertices_enu.len(), 4);
    }
    #[test]
    #[cfg(feature = "experimental")]
    fn test_spatial_rtree_crate_export() {
        let mut tree = crate::SpatialRTree::new();
        tree.insert(crate::SpatialEnvelope::from_point(0.0, 0.0), 42);
        assert_eq!(tree.len(), 1);
    }
    #[test]
    fn sanity() {
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    }
}
pub mod track_assoc;
pub use track_assoc::{
    compute_spatial_matrix_2d, compute_spatial_matrix_3d, euclidean_dist_2d, euclidean_dist_3d,
    solve_nearest_neighbor, solve_optimal_hungarian, AssocConfig, AssocError, AssocMatch,
    AssocResult, DistanceMatrix,
};

#[cfg(feature = "xml-tools")]
pub mod xml_canonicalizer;
#[cfg(feature = "xml-tools")]
pub use xml_canonicalizer::{
    canonicalize_cot_xml, validate_cot_xml, CanonicalizeOptions, CotSchemaValidator,
    CotValidationError, CotXmlCanonicalizer,
};
#[cfg(feature = "experimental")]
pub mod spatial_rtree;
#[cfg(feature = "experimental")]
pub use spatial_rtree::{normalize_lon, RTreeEntry, SpatialEnvelope, SpatialRTree};

#[cfg(feature = "experimental")]
pub mod sidc_parser;
#[cfg(feature = "experimental")]
pub use sidc_parser::{
    cot_type_to_sidc20, generate_sidc2525d, parse_sidc2525d, sidc20_to_cot_type, Echelon,
    MilStd2525DSymbol, SidcParseError, StandardIdentity, SymbolSet, SymbolStatus,
};

#[cfg(feature = "experimental")]
pub mod terrain_sampler;
#[cfg(feature = "experimental")]
pub use terrain_sampler::{
    cot_target_from_intercept, GeoTiffElevationGrid, InterpolationMethod, TerrainIntercept,
    TerrainInterceptSolver, TerrainRay, TerrainSamplerError,
};

#[cfg(feature = "experimental")]
pub mod kalman_target;
#[cfg(feature = "experimental")]
pub use kalman_target::{
    KalmanTargetError, KinematicState, TacticalTargetFilter, TargetFilterConfig,
};

pub mod sax_parser;
pub use sax_parser::{
    CotChunkStreamer, CotEventView, CotPointView, CotSaxParser, SaxAttribute, SaxAttributes,
    SaxAttributesIter, SaxError, SaxEvent, SaxReader,
};

pub mod misb_st0601;
