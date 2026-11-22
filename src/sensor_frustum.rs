//! CoT Sensor FOV Frustum and camera sensor polygon footprint generator.

/// Errors occurring during sensor frustum calculations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrustumError {
    NonFiniteParameter(&'static str),
    InvalidFov(&'static str),
    InvalidAltitude(&'static str),
    InvalidLatitude(&'static str),
    InvalidElevation(&'static str),
    InvalidRange(&'static str),
    NoGroundIntersection,
    DegenerateFrustum,
    XmlParseError(String),
}

impl core::fmt::Display for FrustumError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFiniteParameter(s) => write!(f, "parameter is non-finite: {s}"),
            Self::InvalidFov(s) => write!(f, "invalid FOV angle: {s}"),
            Self::InvalidAltitude(s) => write!(f, "invalid altitude: {s}"),
            Self::InvalidLatitude(s) => write!(f, "invalid latitude: {s}"),
            Self::InvalidElevation(s) => write!(f, "invalid elevation: {s}"),
            Self::InvalidRange(s) => write!(f, "invalid range: {s}"),
            Self::NoGroundIntersection => write!(f, "sensor frustum does not intersect ground"),
            Self::DegenerateFrustum => write!(f, "degenerate sensor frustum geometry"),
            Self::XmlParseError(s) => write!(f, "XML parse error: {s}"),
        }
    }
}

impl std::error::Error for FrustumError {}

/// Position and orientation of the sensor host platform.
/// Heading is clockwise from North (degrees).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlatformPose {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub alt_hae_m: f64,
    pub heading_deg: f64,
    pub pitch_deg: f64,
    pub roll_deg: f64,
}

impl PlatformPose {
    /// Create and validate a new PlatformPose.
    pub fn new(
        lat: f64,
        lon: f64,
        alt: f64,
        hdg: f64,
        pitch: f64,
        roll: f64,
    ) -> Result<Self, FrustumError> {
        if !lat.is_finite()
            || !lon.is_finite()
            || !alt.is_finite()
            || !hdg.is_finite()
            || !pitch.is_finite()
            || !roll.is_finite()
        {
            return Err(FrustumError::NonFiniteParameter("platform pose"));
        }
        if !(-90.0..=90.0).contains(&lat) {
            return Err(FrustumError::InvalidLatitude("latitude out of range"));
        }
        let hdg_norm = (hdg % 360.0 + 360.0) % 360.0;
        Ok(Self {
            lat_deg: lat,
            lon_deg: lon,
            alt_hae_m: alt,
            heading_deg: hdg_norm,
            pitch_deg: pitch,
            roll_deg: roll,
        })
    }
}

/// Gimbal pointing angles relative to platform frame or North-referenced.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorGimbal {
    pub azimuth_deg: f64,
    pub elevation_deg: f64,
    pub roll_deg: f64,
}

impl SensorGimbal {
    /// Create and validate a SensorGimbal pointing angles.
    pub fn new(az: f64, el: f64, roll: f64) -> Result<Self, FrustumError> {
        if !az.is_finite() || !el.is_finite() || !roll.is_finite() {
            return Err(FrustumError::NonFiniteParameter("sensor gimbal"));
        }
        if !(-90.0..=90.0).contains(&el) {
            return Err(FrustumError::InvalidElevation("elevation [-90, 90]"));
        }
        let az_norm = (az % 360.0 + 360.0) % 360.0;
        Ok(Self {
            azimuth_deg: az_norm,
            elevation_deg: el,
            roll_deg: roll,
        })
    }
}

/// Configuration parameters defining the sensor field-of-view frustum.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorFrustumConfig {
    pub h_fov_deg: f64,
    pub v_fov_deg: f64,
    pub max_range_m: f64,
    pub min_range_m: f64,
}

impl SensorFrustumConfig {
    /// Create and validate a new SensorFrustumConfig.
    pub fn new(h_fov: f64, v_fov: f64, max_rng: f64, min_rng: f64) -> Result<Self, FrustumError> {
        if !h_fov.is_finite() || !v_fov.is_finite() || !max_rng.is_finite() || !min_rng.is_finite()
        {
            return Err(FrustumError::NonFiniteParameter("frustum config"));
        }
        if h_fov <= 0.0 || h_fov >= 180.0 {
            return Err(FrustumError::InvalidFov("horizontal FOV in (0, 180)"));
        }
        if v_fov <= 0.0 || v_fov >= 180.0 {
            return Err(FrustumError::InvalidFov("vertical FOV in (0, 180)"));
        }
        if min_rng < 0.0 || max_rng <= min_rng {
            return Err(FrustumError::InvalidRange(
                "max_range must be > min_range >= 0",
            ));
        }
        Ok(Self {
            h_fov_deg: h_fov,
            v_fov_deg: v_fov,
            max_range_m: max_rng,
            min_range_m: min_rng,
        })
    }
}

/// Compute 3x3 rotation matrix from yaw (azimuth clockwise from North), pitch, and roll in ENU.
pub fn euler_to_rot_matrix(yaw_deg: f64, pitch_deg: f64, roll_deg: f64) -> [[f64; 3]; 3] {
    let (s_y, c_y) = yaw_deg.to_radians().sin_cos();
    let (s_p, c_p) = pitch_deg.to_radians().sin_cos();
    let (s_r, c_r) = roll_deg.to_radians().sin_cos();
    [
        [
            c_y * c_r + s_y * s_p * s_r,
            -s_y * c_p,
            c_y * s_r - s_y * s_p * c_r,
        ],
        [
            s_y * c_r - c_y * s_p * s_r,
            c_y * c_p,
            s_y * s_r + c_y * s_p * c_r,
        ],
        [-c_p * s_r, -s_p, c_p * c_r],
    ]
}

/// Multiply a 3x3 matrix by a 3D vector.
pub fn rotate_vec(m: &[[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

/// Intersect a ray with the flat ground plane (Z = 0 in local ENU).
pub fn intersect_ray_ground(
    ray_enu: [f64; 3],
    sensor_h: f64,
    max_range_m: f64,
) -> Result<([f64; 3], f64), FrustumError> {
    if sensor_h <= 0.0 {
        return Err(FrustumError::InvalidAltitude(
            "sensor altitude must be above ground",
        ));
    }
    if ray_enu[2] >= -1e-6 {
        return Err(FrustumError::NoGroundIntersection);
    }
    let t = -sensor_h / ray_enu[2];
    if t > max_range_m {
        return Err(FrustumError::InvalidRange("intersection exceeds max range"));
    }
    let p = [t * ray_enu[0], t * ray_enu[1], 0.0];
    Ok((p, t))
}

/// Ground footprint polygon generated by sensor frustum intersection.
#[derive(Debug, Clone, PartialEq)]
pub struct FrustumFootprint {
    pub vertices_enu: Vec<[f64; 3]>,
    pub vertices_geo: Vec<[f64; 3]>,
    pub boresight_geo: [f64; 3],
    pub area_sq_m: f64,
    pub perimeter_m: f64,
    pub slant_range_m: f64,
}

impl FrustumFootprint {
    /// Calculate centroid in geodetic coordinates [lat, lon, alt].
    pub fn centroid_geo(&self) -> [f64; 3] {
        let n = self.vertices_geo.len() as f64;
        let lat = self.vertices_geo.iter().map(|p| p[0]).sum::<f64>() / n;
        let ref_lon = self.vertices_geo.first().map(|p| p[1]).unwrap_or(0.0);
        let sum_unwrapped: f64 = self
            .vertices_geo
            .iter()
            .map(|p| ref_lon + (p[1] - ref_lon + 180.0).rem_euclid(360.0) - 180.0)
            .sum();
        let lon = (sum_unwrapped / n + 180.0).rem_euclid(360.0) - 180.0;
        let alt = self.vertices_geo.iter().map(|p| p[2]).sum::<f64>() / n;
        [lat, lon, alt]
    }
}

/// Compute 2D polygon area in square meters using Shoelace formula on ENU coordinates.
pub fn polygon_area_enu(pts: &[[f64; 3]]) -> f64 {
    if pts.len() < 3 {
        return 0.0;
    }
    let mut area = 0.0;
    for i in 0..pts.len() {
        let j = (i + 1) % pts.len();
        area += pts[i][0] * pts[j][1] - pts[j][0] * pts[i][1];
    }
    (area * 0.5).abs()
}

/// Compute 2D polygon perimeter in meters on ENU coordinates.
pub fn polygon_perimeter_enu(pts: &[[f64; 3]]) -> f64 {
    if pts.len() < 2 {
        return 0.0;
    }
    let mut p = 0.0;
    for i in 0..pts.len() {
        let j = (i + 1) % pts.len();
        let (dx, dy) = (pts[j][0] - pts[i][0], pts[j][1] - pts[i][1]);
        p += (dx * dx + dy * dy).sqrt();
    }
    p
}

/// Test whether an ENU point [x, y, _] lies inside the 2D polygon footprint.
pub fn point_in_polygon_enu(pt: [f64; 2], poly: &[[f64; 3]]) -> bool {
    if poly.len() < 3 {
        return false;
    }
    let (x, y) = (pt[0], pt[1]);
    let mut inside = false;
    let mut j = poly.len() - 1;
    for i in 0..poly.len() {
        let (xi, yi) = (poly[i][0], poly[i][1]);
        let (xj, yj) = (poly[j][0], poly[j][1]);
        let intersect =
            ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi + 1e-15) + xi);
        if intersect {
            inside = !inside;
        }
        j = i;
    }
    inside
}

impl FrustumFootprint {
    /// Test whether a local ENU point lies inside the ground footprint polygon.
    pub fn contains_enu_point(&self, east_m: f64, north_m: f64) -> bool {
        point_in_polygon_enu([east_m, north_m], &self.vertices_enu)
    }
}

/// Compute boresight and 4 corner ray unit vectors in ENU coordinates.
pub fn compute_frustum_rays_enu(
    az_deg: f64,
    el_deg: f64,
    roll_deg: f64,
    h_fov_deg: f64,
    v_fov_deg: f64,
) -> ([f64; 3], [[f64; 3]; 4]) {
    let (az_rad, el_rad, roll_rad) = (
        az_deg.to_radians(),
        el_deg.to_radians(),
        roll_deg.to_radians(),
    );
    let (cos_el, sin_el) = (el_rad.cos(), el_rad.sin());
    let b = [az_rad.sin() * cos_el, az_rad.cos() * cos_el, sin_el];
    let (r0, u0) = (
        [az_rad.cos(), -az_rad.sin(), 0.0],
        [az_rad.sin() * sin_el, az_rad.cos() * sin_el, -cos_el],
    );
    let (cr, sr) = (roll_rad.cos(), roll_rad.sin());
    let r = [r0[0] * cr + u0[0] * sr, r0[1] * cr + u0[1] * sr, u0[2] * sr];
    let u = [
        -r0[0] * sr + u0[0] * cr,
        -r0[1] * sr + u0[1] * cr,
        u0[2] * cr,
    ];
    let (th, tv) = (
        (h_fov_deg * 0.5).to_radians().tan(),
        (v_fov_deg * 0.5).to_radians().tan(),
    );
    let deltas = [(-th, tv), (th, tv), (th, -tv), (-th, -tv)];
    let mut corners = [[0.0; 3]; 4];
    for (i, (dx, dy)) in deltas.iter().enumerate() {
        let v = [
            b[0] + r[0] * dx + u[0] * dy,
            b[1] + r[1] * dx + u[1] * dy,
            b[2] + r[2] * dx + u[2] * dy,
        ];
        let m = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        corners[i] = [v[0] / m, v[1] / m, v[2] / m];
    }
    (b, corners)
}

/// Compute ENU boresight and 4 corner ground intersection points.
#[allow(clippy::type_complexity)]
pub fn compute_frustum_ground_enu(
    platform: &PlatformPose,
    gimbal: &SensorGimbal,
    config: &SensorFrustumConfig,
    ground_alt_hae_m: f64,
) -> Result<([f64; 3], f64, Vec<[f64; 3]>), FrustumError> {
    let sensor_h = platform.alt_hae_m - ground_alt_hae_m;
    let (bg, corner_rays) = compute_frustum_rays_enu(
        gimbal.azimuth_deg,
        gimbal.elevation_deg,
        gimbal.roll_deg,
        config.h_fov_deg,
        config.v_fov_deg,
    );
    let rot = euler_to_rot_matrix(platform.heading_deg, -platform.pitch_deg, platform.roll_deg);
    let b_enu = rotate_vec(&rot, bg);
    let (b_pt, slant_range) = intersect_ray_ground(b_enu, sensor_h, config.max_range_m)?;
    let mut corners_enu = Vec::with_capacity(4);
    for c_ray in corner_rays {
        let r_enu = rotate_vec(&rot, c_ray);
        let c_pt = if r_enu[2] >= -1e-6 {
            [
                config.max_range_m * r_enu[0],
                config.max_range_m * r_enu[1],
                0.0,
            ]
        } else {
            let t = (-sensor_h / r_enu[2]).clamp(config.min_range_m, config.max_range_m);
            [t * r_enu[0], t * r_enu[1], 0.0]
        };
        corners_enu.push(c_pt);
    }
    Ok((b_pt, slant_range, corners_enu))
}

/// Generate complete sensor ground footprint polygon from platform, gimbal, and sensor configuration.
pub fn generate_sensor_footprint(
    platform: &PlatformPose,
    gimbal: &SensorGimbal,
    config: &SensorFrustumConfig,
    ground_alt_hae_m: f64,
) -> Result<FrustumFootprint, FrustumError> {
    let (b_enu, slant, corners_enu) =
        compute_frustum_ground_enu(platform, gimbal, config, ground_alt_hae_m)?;
    let (b_lat, b_lon, b_alt) =
        crate::camera::enu_to_geodetic(b_enu, platform.lat_deg, platform.lon_deg, ground_alt_hae_m);
    let mut vertices_geo = Vec::with_capacity(corners_enu.len());
    for c in &corners_enu {
        let (lat, lon, alt) = crate::camera::enu_to_geodetic(
            *c,
            platform.lat_deg,
            platform.lon_deg,
            ground_alt_hae_m,
        );
        vertices_geo.push([lat, lon, alt]);
    }
    let area = polygon_area_enu(&corners_enu);
    let perim = polygon_perimeter_enu(&corners_enu);
    Ok(FrustumFootprint {
        vertices_enu: corners_enu,
        vertices_geo,
        boresight_geo: [b_lat, b_lon, b_alt],
        area_sq_m: area,
        perimeter_m: perim,
        slant_range_m: slant,
    })
}

impl FrustumFootprint {
    /// Format the ground footprint polygon as a CoT `<shape>` XML element string.
    pub fn to_cot_shape_xml(&self, stroke_color: i32, fill_color: i32) -> String {
        let mut inner = String::new();
        for pt in &self.vertices_geo {
            inner.push_str(&format!(
                "<vertex lat=\"{:.6}\" lon=\"{:.6}\" hae=\"{:.2}\"/>",
                pt[0], pt[1], pt[2]
            ));
        }
        format!(
            "<shape><polyline closed=\"true\" color=\"{stroke_color}\" fillColor=\"{fill_color}\">{inner}</polyline></shape>"
        )
    }
}

impl SensorGimbal {
    /// Format sensor gimbal pointing as `<sensor .../>` XML element string.
    pub fn to_cot_sensor_xml(&self, fov_deg: f64, range_m: f64) -> String {
        format!(
            "<sensor azimuth=\"{:.2}\" elevation=\"{:.2}\" fov=\"{:.2}\" range=\"{:.2}\"/>",
            self.azimuth_deg, self.elevation_deg, fov_deg, range_m
        )
    }
}

impl FrustumFootprint {
    /// Render complete standalone CoT 2.0 XML event containing the sensor footprint.
    pub fn to_cot_event_xml(
        &self,
        uid: &str,
        callsign: &str,
        time_ms: i64,
        stale_ms: i64,
    ) -> String {
        let [c_lat, c_lon, c_alt] = self.centroid_geo();
        let time_str = crate::cot::format_iso8601_utc(time_ms);
        let stale_str = crate::cot::format_iso8601_utc(stale_ms);
        let shape_xml = self.to_cot_shape_xml(-16711936, 536936192);
        let cs_esc = crate::cot::escape_xml_attr(callsign);
        let uid_esc = crate::cot::escape_xml_attr(uid);
        format!(
            "<event version=\"2.0\" uid=\"{uid_esc}\" type=\"u-d-f\" how=\"m-g\" time=\"{time_str}\" start=\"{time_str}\" stale=\"{stale_str}\">\
<point lat=\"{c_lat:.7}\" lon=\"{c_lon:.7}\" hae=\"{c_alt:.2}\" ce=\"10.0\" le=\"10.0\"/>\
<detail><contact callsign=\"{cs_esc}\"/>{shape_xml}</detail></event>"
        )
    }
}

fn extract_xml_attr(tag: &str, attr: &str) -> Option<String> {
    let needle = format!(" {attr}=\"");
    let start = tag.find(&needle)? + needle.len();
    let end = tag[start..].find('"')?;
    Some(tag[start..start + end].to_string())
}

impl SensorGimbal {
    /// Parse `<sensor .../>` XML string into SensorGimbal and (fov, range).
    pub fn parse_cot_sensor(xml: &str) -> Result<(Self, f64, f64), FrustumError> {
        let tag_start = xml
            .find("<sensor")
            .ok_or_else(|| FrustumError::XmlParseError("missing <sensor".into()))?;
        let tag_end = xml[tag_start..]
            .find('>')
            .ok_or_else(|| FrustumError::XmlParseError("unclosed <sensor".into()))?;
        let tag = &xml[tag_start..tag_start + tag_end + 1];
        let az = extract_xml_attr(tag, "azimuth")
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| FrustumError::XmlParseError("azimuth".into()))?;
        let el = extract_xml_attr(tag, "elevation")
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| FrustumError::XmlParseError("elevation".into()))?;
        let fov: f64 = extract_xml_attr(tag, "fov")
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| FrustumError::XmlParseError("fov".into()))?;
        let rng: f64 = extract_xml_attr(tag, "range")
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| FrustumError::XmlParseError("range".into()))?;
        if !fov.is_finite() || !rng.is_finite() {
            return Err(FrustumError::NonFiniteParameter("fov or range"));
        }
        let gimbal = Self::new(az, el, 0.0)?;
        Ok((gimbal, fov, rng))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frustum_parameter_validation() {
        assert_eq!(
            PlatformPose::new(f64::NAN, 0.0, 100.0, 0.0, 0.0, 0.0),
            Err(FrustumError::NonFiniteParameter("platform pose"))
        );
        assert_eq!(
            PlatformPose::new(95.0, 0.0, 100.0, 0.0, 0.0, 0.0),
            Err(FrustumError::InvalidLatitude("latitude out of range"))
        );
        assert_eq!(
            SensorGimbal::new(0.0, -95.0, 0.0),
            Err(FrustumError::InvalidElevation("elevation [-90, 90]"))
        );
        assert_eq!(
            SensorFrustumConfig::new(0.0, 45.0, 1000.0, 0.0),
            Err(FrustumError::InvalidFov("horizontal FOV in (0, 180)"))
        );
        assert_eq!(
            SensorFrustumConfig::new(190.0, 45.0, 1000.0, 0.0),
            Err(FrustumError::InvalidFov("horizontal FOV in (0, 180)"))
        );
        assert_eq!(
            SensorFrustumConfig::new(60.0, 45.0, 100.0, 200.0),
            Err(FrustumError::InvalidRange(
                "max_range must be > min_range >= 0"
            ))
        );
    }

    #[test]
    fn test_nadir_footprint_generation() {
        let platform = PlatformPose::new(0.0, 0.0, 1000.0, 0.0, 0.0, 0.0).unwrap();
        let gimbal = SensorGimbal::new(0.0, -90.0, 0.0).unwrap();
        let cfg = SensorFrustumConfig::new(60.0, 60.0, 5000.0, 10.0).unwrap();
        let fp = generate_sensor_footprint(&platform, &gimbal, &cfg, 0.0).unwrap();
        assert_eq!(fp.vertices_enu.len(), 4);
        assert!((fp.slant_range_m - 1000.0).abs() < 1e-3);
        assert!(fp.area_sq_m > 0.0);
        assert!(fp.perimeter_m > 0.0);
    }

    #[test]
    fn test_oblique_footprint_and_containment() {
        let platform = PlatformPose::new(0.0, 0.0, 500.0, 0.0, 0.0, 0.0).unwrap();
        let gimbal = SensorGimbal::new(0.0, -45.0, 0.0).unwrap();
        let cfg = SensorFrustumConfig::new(30.0, 30.0, 2000.0, 5.0).unwrap();
        let fp = generate_sensor_footprint(&platform, &gimbal, &cfg, 0.0).unwrap();
        assert_eq!(fp.vertices_enu.len(), 4);
        assert!(fp.contains_enu_point(0.0, 500.0));
        assert!(!fp.contains_enu_point(0.0, -500.0));
    }

    #[test]
    fn test_cot_xml_roundtrip() {
        let platform = PlatformPose::new(37.7749, -122.4194, 800.0, 45.0, 0.0, 0.0).unwrap();
        let gimbal = SensorGimbal::new(10.0, -30.0, 0.0).unwrap();
        let cfg = SensorFrustumConfig::new(40.0, 30.0, 4000.0, 1.0).unwrap();
        let fp = generate_sensor_footprint(&platform, &gimbal, &cfg, 10.0).unwrap();
        let xml = fp.to_cot_event_xml("UID-1", "DRONE-1", 1700000000000, 1700000060000);
        assert!(xml.contains("DRONE-1"));
        assert!(xml.contains("<shape><polyline"));
        let sensor_xml = gimbal.to_cot_sensor_xml(cfg.h_fov_deg, cfg.max_range_m);
        let (parsed_g, parsed_fov, _) = SensorGimbal::parse_cot_sensor(&sensor_xml).unwrap();
        assert!((parsed_g.azimuth_deg - gimbal.azimuth_deg).abs() < 1e-2);
        assert!((parsed_fov - cfg.h_fov_deg).abs() < 1e-2);
    }
}
