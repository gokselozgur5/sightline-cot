//! Geodesic polygon area calculator for Cursor-on-Target (CoT) tactical zones.

/// Errors encountered when computing geodesic polygon area.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeodesicAreaError {
    InsufficientVertices,
    NonFiniteCoordinate,
    LatitudeOutOfRange,
    LongitudeOutOfRange,
    DegeneratePolygon,
}

impl core::fmt::Display for GeodesicAreaError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InsufficientVertices => write!(f, "at least 3 vertices required"),
            Self::NonFiniteCoordinate => write!(f, "coordinate is not finite"),
            Self::LatitudeOutOfRange => write!(f, "latitude must be within [-90, 90] degrees"),
            Self::LongitudeOutOfRange => write!(f, "longitude must be within [-180, 180] degrees"),
            Self::DegeneratePolygon => write!(f, "polygon is degenerate with zero or undefined area"),
        }
    }
}

impl std::error::Error for GeodesicAreaError {}

/// WGS84 semi-major axis in meters.
pub const WGS84_A_M: f64 = 6_378_137.0;

/// WGS84 flattening.
pub const WGS84_F: f64 = 1.0 / 298.257_223_563;

/// WGS84 first eccentricity squared (e^2 = 2f - f^2).
pub const WGS84_E2: f64 = 0.006_694_379_990_141_316_5;

/// WGS84 authalic sphere radius in meters (R_q = sqrt(A / 4pi)).
pub const WGS84_AUTHALIC_RADIUS_M: f64 = 6_371_007.180_918_474;

/// Compute the authalic parameter q(phi) for the WGS84 ellipsoid.
pub fn authalic_q(lat_rad: f64) -> f64 {
    let sin_phi = lat_rad.sin();
    let e = WGS84_E2.sqrt();
    let e_sin = e * sin_phi;
    let term1 = sin_phi / (1.0 - WGS84_E2 * sin_phi * sin_phi);
    let term2 = ((1.0 - e_sin) / (1.0 + e_sin)).ln() / (2.0 * e);
    (1.0 - WGS84_E2) * (term1 - term2)
}

/// Compute sin(beta) where beta is the authalic latitude on WGS84.
pub fn authalic_sin_beta(lat_deg: f64) -> Result<f64, GeodesicAreaError> {
    if !lat_deg.is_finite() {
        return Err(GeodesicAreaError::NonFiniteCoordinate);
    }
    if !(-90.0..=90.0).contains(&lat_deg) {
        return Err(GeodesicAreaError::LatitudeOutOfRange);
    }
    let q = authalic_q(lat_deg.to_radians());
    let q_polar = authalic_q(core::f64::consts::FRAC_PI_2);
    Ok((q / q_polar).clamp(-1.0, 1.0))
}

/// Wrap longitude difference into [-PI, PI] radians.
pub fn wrap_delta_lon(dlon_rad: f64) -> f64 {
    let pi = core::f64::consts::PI;
    let two_pi = 2.0 * pi;
    let mut rem = dlon_rad % two_pi;
    if rem > pi {
        rem -= two_pi;
    } else if rem < -pi {
        rem += two_pi;
    }
    rem
}

/// Compute geodesic area of a polygon on the WGS84 ellipsoid in square meters.
pub fn geodesic_polygon_area(vertices: &[[f64; 2]]) -> Result<f64, GeodesicAreaError> {
    let n = vertices.len();
    if n < 3 {
        return Err(GeodesicAreaError::InsufficientVertices);
    }
    for p in vertices {
        if !p[0].is_finite() || !p[1].is_finite() {
            return Err(GeodesicAreaError::NonFiniteCoordinate);
        }
        if !(-90.0..=90.0).contains(&p[0]) {
            return Err(GeodesicAreaError::LatitudeOutOfRange);
        }
        if !(-180.0..=180.0).contains(&p[1]) {
            return Err(GeodesicAreaError::LongitudeOutOfRange);
        }
    }
    let mut sum = 0.0;
    let mut dlon_sum = 0.0;
    let mut lat_sum = 0.0;
    for i in 0..n {
        let (p1, p2) = (vertices[i], vertices[(i + 1) % n]);
        lat_sum += p1[0];
        let sin_beta1 = authalic_sin_beta(p1[0])?;
        let sin_beta2 = authalic_sin_beta(p2[0])?;
        let dlon = wrap_delta_lon((p2[1] - p1[1]).to_radians());
        dlon_sum += dlon;
        sum += dlon * (2.0 + sin_beta1 + sin_beta2);
    }
    let r = WGS84_AUTHALIC_RADIUS_M;
    let mut area = (0.5 * r * r * sum).abs();
    if (dlon_sum.abs() - 2.0 * core::f64::consts::PI).abs() < 1e-6 && lat_sum > 0.0 {
        area = (4.0 * core::f64::consts::PI * r * r - area).abs();
    }
    Ok(area)
}

/// Comprehensive geometry metrics for a polygonal CoT zone.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeodesicPolygonArea {
    pub area_m2: f64,
    pub perimeter_m: f64,
}

impl GeodesicPolygonArea {
    /// Area in square kilometers.
    pub fn area_km2(&self) -> f64 {
        self.area_m2 / 1_000_000.0
    }

    /// Area in hectares (1 ha = 10,000 m^2).
    pub fn area_hectares(&self) -> f64 {
        self.area_m2 / 10_000.0
    }

    /// Area in acres (1 acre = 4046.8564224 m^2).
    pub fn area_acres(&self) -> f64 {
        self.area_m2 / 4_046.856_422_4
    }
}

/// Compute polygon perimeter along great-circle arcs in meters.
pub fn polygon_geodesic_perimeter(vertices: &[[f64; 2]]) -> Result<f64, GeodesicAreaError> {
    let n = vertices.len();
    if n < 3 {
        return Err(GeodesicAreaError::InsufficientVertices);
    }
    let mut total_rad = 0.0;
    for i in 0..n {
        let (p1, p2) = (vertices[i], vertices[(i + 1) % n]);
        let (phi1, phi2) = (p1[0].to_radians(), p2[0].to_radians());
        let dphi = phi2 - phi1;
        let dlam = wrap_delta_lon((p2[1] - p1[1]).to_radians());
        let h = (dphi * 0.5).sin().powi(2) + phi1.cos() * phi2.cos() * (dlam * 0.5).sin().powi(2);
        let c = 2.0 * h.clamp(0.0, 1.0).sqrt().asin();
        total_rad += c;
    }
    Ok(total_rad * WGS84_AUTHALIC_RADIUS_M)
}

/// Compute full geometry metrics for a polygonal CoT zone.
pub fn compute_cot_polygon_geometry(
    vertices: &[[f64; 2]],
) -> Result<GeodesicPolygonArea, GeodesicAreaError> {
    let area_m2 = geodesic_polygon_area(vertices)?;
    let perimeter_m = polygon_geodesic_perimeter(vertices)?;
    Ok(GeodesicPolygonArea {
        area_m2,
        perimeter_m,
    })
}

/// Check whether polygon vertices are ordered counter-clockwise (positive signed area).
pub fn is_counter_clockwise(vertices: &[[f64; 2]]) -> Result<bool, GeodesicAreaError> {
    let n = vertices.len();
    if n < 3 {
        return Err(GeodesicAreaError::InsufficientVertices);
    }
    let mut sum = 0.0;
    for i in 0..n {
        let (p1, p2) = (vertices[i], vertices[(i + 1) % n]);
        let sin_beta1 = authalic_sin_beta(p1[0])?;
        let sin_beta2 = authalic_sin_beta(p2[0])?;
        let dlon = wrap_delta_lon((p2[1] - p1[1]).to_radians());
        sum += dlon * (2.0 + sin_beta1 + sin_beta2);
    }
    if sum.abs() < 1e-15 {
        return Err(GeodesicAreaError::DegeneratePolygon);
    }
    Ok(sum > 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authalic_latitude_poles_and_equator() {
        let q_eq = authalic_q(0.0);
        assert!(q_eq.abs() < 1e-12);
        let sin_beta_eq = authalic_sin_beta(0.0).unwrap();
        assert!(sin_beta_eq.abs() < 1e-12);
        let sin_beta_np = authalic_sin_beta(90.0).unwrap();
        assert!((sin_beta_np - 1.0).abs() < 1e-12);
        let sin_beta_sp = authalic_sin_beta(-90.0).unwrap();
        assert!((sin_beta_sp - (-1.0)).abs() < 1e-12);
    }

    #[test]
    fn test_equatorial_square_area() {
        let square = [
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ];
        let area = geodesic_polygon_area(&square).unwrap();
        assert!((area - 1.2308e10).abs() < 5e7);
    }

    #[test]
    fn test_triangular_cot_geofence() {
        let tri = [
            [37.7749, -122.4194],
            [37.7749, -122.4094],
            [37.7849, -122.4194],
        ];
        let geom = compute_cot_polygon_geometry(&tri).unwrap();
        assert!(geom.area_m2 > 400_000.0 && geom.area_m2 < 600_000.0);
        assert!(geom.perimeter_m > 2500.0 && geom.perimeter_m < 3500.0);
        assert!(geom.area_km2() > 0.4 && geom.area_km2() < 0.6);
        assert!(geom.area_hectares() > 40.0 && geom.area_hectares() < 60.0);
    }

    #[test]
    fn test_antimeridian_crossing_area() {
        let poly = [
            [10.0, 179.0],
            [10.0, -179.0],
            [11.0, -179.0],
            [11.0, 179.0],
        ];
        let area = geodesic_polygon_area(&poly).unwrap();
        assert!(area > 2.0e10 && area < 3.0e10);
    }

    #[test]
    fn test_polygon_winding_order() {
        let ccw = [
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ];
        let cw = [
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 1.0],
            [1.0, 0.0],
        ];
        assert_eq!(is_counter_clockwise(&ccw), Ok(true));
        assert_eq!(is_counter_clockwise(&cw), Ok(false));
    }

    #[test]
    fn test_coordinate_validation_errors() {
        let nan_poly = [[f64::NAN, 0.0], [1.0, 0.0], [1.0, 1.0]];
        assert_eq!(
            geodesic_polygon_area(&nan_poly),
            Err(GeodesicAreaError::NonFiniteCoordinate)
        );
        let lat_oob = [[95.0, 0.0], [1.0, 0.0], [1.0, 1.0]];
        assert_eq!(
            geodesic_polygon_area(&lat_oob),
            Err(GeodesicAreaError::LatitudeOutOfRange)
        );
        let lon_oob = [[0.0, 190.0], [1.0, 0.0], [1.0, 1.0]];
        assert_eq!(
            geodesic_polygon_area(&lon_oob),
            Err(GeodesicAreaError::LongitudeOutOfRange)
        );
    }

    #[test]
    fn test_insufficient_vertices() {
        let empty: &[[f64; 2]] = &[];
        assert_eq!(
            geodesic_polygon_area(empty),
            Err(GeodesicAreaError::InsufficientVertices)
        );
        let pair = [[0.0, 0.0], [1.0, 1.0]];
        assert_eq!(
            geodesic_polygon_area(&pair),
            Err(GeodesicAreaError::InsufficientVertices)
        );
        assert_eq!(
            polygon_geodesic_perimeter(&pair),
            Err(GeodesicAreaError::InsufficientVertices)
        );
    }
}
