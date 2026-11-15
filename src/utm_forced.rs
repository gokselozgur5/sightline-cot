//! UTM projection forced into a chosen zone across boundaries.

use crate::mgrs::{GridError, UtmCoord};

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

fn normalize_dlon_rad(mut dlon: f64) -> f64 {
    while dlon > std::f64::consts::PI {
        dlon -= 2.0 * std::f64::consts::PI;
    }
    while dlon < -std::f64::consts::PI {
        dlon += 2.0 * std::f64::consts::PI;
    }
    dlon
}

fn meridian_arc(phi: f64, e2: f64) -> f64 {
    let e4 = e2 * e2;
    let e6 = e4 * e2;
    let a0 = 1.0 - e2 / 4.0 - 3.0 * e4 / 64.0 - 5.0 * e6 / 256.0;
    let a2 = 3.0 * e2 / 8.0 + 3.0 * e4 / 32.0 + 45.0 * e6 / 1024.0;
    let a4 = 15.0 * e4 / 256.0 + 45.0 * e6 / 1024.0;
    let a6 = 35.0 * e6 / 3072.0;
    WGS84_A * (a0 * phi - a2 * (2.0 * phi).sin() + a4 * (4.0 * phi).sin() - a6 * (6.0 * phi).sin())
}

fn compute_en_terms(lat: f64, dlon: f64, nu: f64, m: f64, ep2: f64) -> (f64, f64) {
    let cos_lat = lat.cos();
    let (t, eta2) = (lat.tan() * lat.tan(), ep2 * cos_lat * cos_lat);
    let (p, p2) = (dlon * cos_lat, (dlon * cos_lat).powi(2));
    let (p3, p4, p5, p6, p7) = (p * p2, p2 * p2, p * p2 * p2, p2 * p2 * p2, p * p2 * p2 * p2);
    let e = FALSE_EASTING + UTM_K0 * nu * (p + (1.0 - t + eta2) * p3 / 6.0
        + (5.0 - 18.0 * t + t * t + 72.0 * eta2) * p5 / 120.0
        + (61.0 - 479.0 * t + 179.0 * t * t - t * t * t) * p7 / 5040.0);
    let n = UTM_K0 * (m + nu * lat.tan() * (p2 / 2.0
        + (5.0 - t + 9.0 * eta2 + 4.0 * eta2 * eta2) * p4 / 24.0
        + (61.0 - 58.0 * t + t * t + 600.0 * eta2) * p6 / 720.0));
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
    let north = if hemi == 'S' { FALSE_NORTHING_SOUTH + north_val } else { north_val };
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

fn find_footpoint_latitude(m: f64, e2: f64) -> f64 {
    let mut phi = m / (WGS84_A * (1.0 - e2 / 4.0));
    for _ in 0..6 {
        let m_eval = meridian_arc(phi, e2);
        let d = (1.0 - e2 * phi.sin().powi(2)).sqrt();
        let dm = WGS84_A * (1.0 - e2) / (d * d * d);
        let diff = (m - m_eval) / dm;
        phi += diff;
        if diff.abs() < 1e-12 {
            break;
        }
    }
    phi
}
