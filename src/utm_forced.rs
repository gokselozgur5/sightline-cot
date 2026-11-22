//! UTM projection forced into a chosen zone across boundaries.

use crate::mgrs::{find_footpoint_latitude, meridian_arc, GridError, UtmCoord};

const WGS84_A: f64 = 6378137.0;
const WGS84_F: f64 = 1.0 / 298.257223563;
const UTM_K0: f64 = 0.9996;
const FALSE_EASTING: f64 = 500000.0;
const FALSE_NORTHING_SOUTH: f64 = 10000000.0;

/// Return central meridian for a given UTM zone in radians.
#[inline]
pub fn zone_central_meridian_rad(zone: u8) -> f64 {
    ((zone as f64 - 1.0) * 6.0 - 180.0 + 3.0).to_radians()
}

fn normalize_dlon_rad(dlon: f64) -> f64 {
    (dlon + std::f64::consts::PI).rem_euclid(2.0 * std::f64::consts::PI) - std::f64::consts::PI
}

fn compute_en_terms(lat: f64, dlon: f64, nu: f64, m: f64, ep2: f64) -> (f64, f64) {
    let cos_lat = lat.cos();
    let (t, eta2) = (lat.tan() * lat.tan(), ep2 * cos_lat * cos_lat);
    let (p, p2) = (dlon * cos_lat, (dlon * cos_lat).powi(2));
    let (p3, p4, p5, p6, p7) = (p * p2, p2 * p2, p * p2 * p2, p2 * p2 * p2, p * p2 * p2 * p2);
    let e = FALSE_EASTING
        + UTM_K0
            * nu
            * (p + (1.0 - t + eta2) * p3 / 6.0
                + (5.0 - 18.0 * t + t * t + 72.0 * eta2 - 58.0 * ep2) * p5 / 120.0
                + (61.0 - 479.0 * t + 179.0 * t * t - t * t * t) * p7 / 5040.0);
    let n = UTM_K0
        * (m + nu
            * lat.tan()
            * (p2 / 2.0
                + (5.0 - t + 9.0 * eta2 + 4.0 * eta2 * eta2) * p4 / 24.0
                + (61.0 - 58.0 * t + t * t + 600.0 * eta2 - 330.0 * ep2) * p6 / 720.0));
    (e, n)
}

fn utm_project_point_forced(lat_deg: f64, lon_deg: f64, zone: u8, hemi: char) -> (f64, f64) {
    let lat = lat_deg.to_radians();
    let lon0 = zone_central_meridian_rad(zone);
    let dlon = normalize_dlon_rad(lon_deg.to_radians() - lon0);
    let sin_lat = lat.sin();
    let e2 = WGS84_F * (2.0 - WGS84_F);
    let ep2 = e2 / (1.0 - e2);
    let nu = WGS84_A / (1.0 - e2 * sin_lat * sin_lat).sqrt();
    let m = meridian_arc(lat, e2);
    let (east, north_val) = compute_en_terms(lat, dlon, nu, m, ep2);
    let north = if hemi == 'S' {
        FALSE_NORTHING_SOUTH + north_val
    } else {
        north_val
    };
    (east, north)
}

/// Project WGS84 lat/lon into a user-specified UTM zone.
///
/// Unlike standard UTM, this allows extending coordinates across adjacent zone boundaries.
pub fn wgs84_to_utm_forced(
    lat_deg: f64,
    lon_deg: f64,
    forced_zone: u8,
) -> Result<UtmCoord, GridError> {
    if !(-80.0..=84.0).contains(&lat_deg) {
        return Err(GridError::InvalidLatitude);
    }
    if !(-180.0..=180.0).contains(&lon_deg) {
        return Err(GridError::InvalidLongitude);
    }
    if !(1..=60).contains(&forced_zone) {
        return Err(GridError::InvalidZone);
    }
    let hemisphere = if lat_deg >= 0.0 { 'N' } else { 'S' };
    let (easting, northing) = utm_project_point_forced(lat_deg, lon_deg, forced_zone, hemisphere);
    Ok(UtmCoord {
        zone: forced_zone,
        hemisphere,
        easting,
        northing,
    })
}

fn unproject_series(
    d: f64,
    t1: f64,
    eta1_2: f64,
    ep2: f64,
    phi1: f64,
    nu1: f64,
    rho1: f64,
) -> (f64, f64) {
    let (d2, d3, d4, d5, d6) = (
        d * d,
        d * d * d,
        d * d * d * d,
        d * d * d * d * d,
        d * d * d * d * d * d,
    );
    let lat = phi1
        - (nu1 * phi1.tan() / rho1)
            * (d2 / 2.0
                - (5.0 + 3.0 * t1 + 10.0 * eta1_2 - 4.0 * eta1_2 * eta1_2 - 9.0 * ep2) * d4 / 24.0
                + (61.0 + 90.0 * t1 + 298.0 * eta1_2 + 45.0 * t1 * t1) * d6 / 720.0
                - (1385.0 + 3633.0 * t1 + 4095.0 * t1 * t1 + 1575.0 * t1 * t1 * t1) * d6 * d2
                    / 40320.0);
    let dlon = (d - (1.0 + 2.0 * t1 + eta1_2) * d3 / 6.0
        + (5.0 - 2.0 * eta1_2 + 28.0 * t1 - 3.0 * eta1_2 * eta1_2 + 8.0 * ep2 + 24.0 * t1 * t1)
            * d5
            / 120.0
        - (61.0 + 662.0 * t1 + 1320.0 * t1 * t1 + 720.0 * t1 * t1 * t1) * d6 * d / 5040.0)
        / phi1.cos();
    (lat, dlon)
}

fn utm_unproject_point_forced(coord: &UtmCoord) -> (f64, f64) {
    let e2 = WGS84_F * (2.0 - WGS84_F);
    let ep2 = e2 / (1.0 - e2);
    let y = if coord.hemisphere == 'S' {
        coord.northing - FALSE_NORTHING_SOUTH
    } else {
        coord.northing
    };
    let x = coord.easting - FALSE_EASTING;
    let m = y / UTM_K0;
    let phi1 = find_footpoint_latitude(m, e2);
    let sin_phi1 = phi1.sin();
    let nu1 = WGS84_A / (1.0 - e2 * sin_phi1 * sin_phi1).sqrt();
    let rho1 = WGS84_A * (1.0 - e2) / (1.0 - e2 * sin_phi1 * sin_phi1).powf(1.5);
    let d = x / (nu1 * UTM_K0);
    let (t1, eta1_2) = (phi1.tan().powi(2), ep2 * phi1.cos().powi(2));
    let (lat_rad, dlon_rad) = unproject_series(d, t1, eta1_2, ep2, phi1, nu1, rho1);
    let lon0 = zone_central_meridian_rad(coord.zone);
    let lon_deg = (lon0 + dlon_rad).to_degrees();
    let lon_norm = ((lon_deg + 180.0).rem_euclid(360.0)) - 180.0;
    (lat_rad.to_degrees(), lon_norm)
}

/// Unproject a UTM coordinate forced into a chosen zone back to WGS84 lat/lon.
///
/// Unlike standard [`crate::mgrs::utm_to_wgs84`], this supports coordinates spanning
/// beyond standard single-zone boundaries.
pub fn utm_forced_to_wgs84(coord: &UtmCoord) -> Result<(f64, f64), GridError> {
    if !(1..=60).contains(&coord.zone) {
        return Err(GridError::InvalidZone);
    }
    if coord.hemisphere != 'N' && coord.hemisphere != 'S' {
        return Err(GridError::InvalidLatitude);
    }
    if !coord.easting.is_finite() {
        return Err(GridError::InvalidEasting);
    }
    if !coord.northing.is_finite() {
        return Err(GridError::InvalidNorthing);
    }
    Ok(utm_unproject_point_forced(coord))
}

/// Compute the shortest zone difference between two UTM zones wrapping around the antimeridian.
pub fn zone_distance_delta(current_zone: u8, target_zone: u8) -> i8 {
    let diff = (target_zone as i16) - (current_zone as i16);
    let wrapped = ((diff + 30).rem_euclid(60)) - 30;
    wrapped as i8
}

/// Return true if two UTM zones are immediate neighbors across borders or antimeridian.
pub fn is_adjacent_zone(zone1: u8, zone2: u8) -> bool {
    zone_distance_delta(zone1, zone2).abs() == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_dlon_rad_large_input_termination() {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let res = normalize_dlon_rad(1e20);
            let _ = tx.send(res);
        });
        let res = rx
            .recv_timeout(std::time::Duration::from_millis(500))
            .expect("must terminate within 500ms");
        assert!(res.abs() <= std::f64::consts::PI);
    }
}
