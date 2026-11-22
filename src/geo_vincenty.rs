//! Vincenty direct and inverse geodesic algorithms on the WGS84 ellipsoid.

/// Errors from Vincenty geodesic computations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VincentyError {
    InvalidLatitude,
    InvalidLongitude,
    NegativeDistance,
    NonFiniteInput,
    NoConvergence,
}

impl core::fmt::Display for VincentyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLatitude => write!(f, "latitude out of range [-90, 90]"),
            Self::InvalidLongitude => write!(f, "longitude out of range [-180, 180]"),
            Self::NegativeDistance => write!(f, "distance cannot be negative"),
            Self::NonFiniteInput => write!(f, "input values must be finite"),
            Self::NoConvergence => write!(f, "Vincenty formula failed to converge"),
        }
    }
}

impl std::error::Error for VincentyError {}

/// Result of the Vincenty inverse geodesic calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VincentyInverse {
    pub distance_m: f64,
    pub forward_azimuth_deg: f64,
    pub reverse_azimuth_deg: f64,
}

/// Result of the Vincenty direct geodesic calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VincentyDirect {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub reverse_azimuth_deg: f64,
}

pub const WGS84_A: f64 = 6_378_137.0;
pub const WGS84_F: f64 = 1.0 / 298.257_223_563;
pub const WGS84_B: f64 = 6_356_752.314_245_179;

pub(crate) fn validate_point(lat_deg: f64, lon_deg: f64) -> Result<(), VincentyError> {
    if !lat_deg.is_finite() || !lon_deg.is_finite() {
        return Err(VincentyError::NonFiniteInput);
    }
    if !(-90.0..=90.0).contains(&lat_deg) {
        return Err(VincentyError::InvalidLatitude);
    }
    if !(-180.0..=180.0).contains(&lon_deg) {
        return Err(VincentyError::InvalidLongitude);
    }
    Ok(())
}

use crate::ellipsoid::Ellipsoid;
pub use crate::geo_vincenty_direct::vincenty_direct_ellipsoid;

pub(crate) fn vincenty_ab_ellipsoid(cos2_alpha: f64, a_semi: f64, b_semi: f64) -> (f64, f64) {
    let u2 = cos2_alpha * (a_semi * a_semi - b_semi * b_semi) / (b_semi * b_semi);
    let a = 1.0 + u2 / 16384.0 * (4096.0 + u2 * (-768.0 + u2 * (320.0 - 175.0 * u2)));
    let b = u2 / 1024.0 * (256.0 + u2 * (-128.0 + u2 * (74.0 - 47.0 * u2)));
    (a, b)
}

pub(crate) fn vincenty_c_ellipsoid(cos2_alpha: f64, f: f64) -> f64 {
    f / 16.0 * cos2_alpha * (4.0 + f * (4.0 - 3.0 * cos2_alpha))
}

pub(crate) fn vincenty_delta_sigma(
    b: f64,
    sin_sigma: f64,
    cos_sigma: f64,
    cos_2sigma_m: f64,
) -> f64 {
    let cos2_2sigma_m = cos_2sigma_m * cos_2sigma_m;
    b * sin_sigma
        * (cos_2sigma_m
            + b / 4.0
                * (cos_sigma * (-1.0 + 2.0 * cos2_2sigma_m)
                    - b / 6.0
                        * cos_2sigma_m
                        * (-3.0 + 4.0 * sin_sigma * sin_sigma)
                        * (-3.0 + 4.0 * cos2_2sigma_m)))
}

fn solve_step(
    sin_u1: f64,
    cos_u1: f64,
    sin_u2: f64,
    cos_u2: f64,
    lambda: f64,
) -> Result<(f64, f64, f64, f64, f64, f64), VincentyError> {
    let (sin_l, cos_l) = (lambda.sin(), lambda.cos());
    let sin_sigma =
        ((cos_u2 * sin_l).powi(2) + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_l).powi(2)).sqrt();
    if sin_sigma.abs() < 1e-12 {
        return Err(VincentyError::NoConvergence);
    }
    let cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_l;
    let sigma = sin_sigma.atan2(cos_sigma);
    let sin_alpha = cos_u1 * cos_u2 * sin_l / sin_sigma;
    let cos2_alpha = 1.0 - sin_alpha * sin_alpha;
    let cos_2sigma_m = if cos2_alpha.abs() > 1e-12 {
        cos_sigma - 2.0 * sin_u1 * sin_u2 / cos2_alpha
    } else {
        0.0
    };
    Ok((
        sigma,
        sin_sigma,
        cos_sigma,
        sin_alpha,
        cos2_alpha,
        cos_2sigma_m,
    ))
}

fn solve_inverse_loop_ellipsoid(
    sin_u1: f64,
    cos_u1: f64,
    sin_u2: f64,
    cos_u2: f64,
    l: f64,
    f: f64,
) -> Result<(f64, f64, f64, f64, f64), VincentyError> {
    let mut lambda = l;
    for _ in 0..200 {
        let (sigma, s_sig, c_sig, s_alp, c2_alp, c_2sm) =
            solve_step(sin_u1, cos_u1, sin_u2, cos_u2, lambda)?;
        let c = vincenty_c_ellipsoid(c2_alp, f);
        let lam_next = l
            + (1.0 - c)
                * f
                * s_alp
                * (sigma + c * s_sig * (c_2sm + c * c_sig * (-1.0 + 2.0 * c_2sm * c_2sm)));
        if (lam_next - lambda).abs() < 1e-12 {
            return Ok((sigma, s_alp, c2_alp, c_2sm, lam_next));
        }
        lambda = lam_next;
    }
    Err(VincentyError::NoConvergence)
}

fn wrap_to_pi(rad: f64) -> f64 {
    (rad + core::f64::consts::PI).rem_euclid(2.0 * core::f64::consts::PI) - core::f64::consts::PI
}

fn wrap_azimuth(deg: f64) -> f64 {
    (deg + 360.0).rem_euclid(360.0)
}

#[allow(clippy::too_many_arguments)]
fn compute_inverse_result_ellipsoid(
    sigma: f64,
    c2_a: f64,
    c_2sm: f64,
    lam: f64,
    s_u1: f64,
    c_u1: f64,
    s_u2: f64,
    c_u2: f64,
    a_semi: f64,
    b_semi: f64,
) -> VincentyInverse {
    let (a, b) = vincenty_ab_ellipsoid(c2_a, a_semi, b_semi);
    let s = b_semi * a * (sigma - vincenty_delta_sigma(b, sigma.sin(), sigma.cos(), c_2sm));
    let a1 = (c_u2 * lam.sin())
        .atan2(c_u1 * s_u2 - s_u1 * c_u2 * lam.cos())
        .to_degrees();
    let a2 = (c_u1 * lam.sin())
        .atan2(-s_u1 * c_u2 + c_u1 * s_u2 * lam.cos())
        .to_degrees();
    VincentyInverse {
        distance_m: s,
        forward_azimuth_deg: wrap_azimuth(a1),
        reverse_azimuth_deg: wrap_azimuth(a2 + 180.0),
    }
}

/// Computes geodesic distance and azimuths between two points on a given ellipsoid.
pub fn vincenty_inverse_ellipsoid(
    lat1: f64,
    lon1: f64,
    lat2: f64,
    lon2: f64,
    ellipsoid: &Ellipsoid,
) -> Result<VincentyInverse, VincentyError> {
    validate_point(lat1, lon1)?;
    validate_point(lat2, lon2)?;
    let same_pole = (lat1 >= 90.0 - 1e-12 && lat2 >= 90.0 - 1e-12)
        || (lat1 <= -90.0 + 1e-12 && lat2 <= -90.0 + 1e-12);
    let dlon = wrap_to_pi((lon2 - lon1).to_radians());
    if ((lat1 - lat2).abs() < 1e-12 && dlon.abs() < 1e-12) || same_pole {
        return Ok(VincentyInverse {
            distance_m: 0.0,
            forward_azimuth_deg: 0.0,
            reverse_azimuth_deg: 0.0,
        });
    }
    let pole_pair = (lat1.abs() >= 90.0 - 1e-12 && lat2.abs() >= 90.0 - 1e-12 && lat1 * lat2 < 0.0)
        || ((lat1 + lat2).abs() < 1e-12
            && (dlon.abs() - core::f64::consts::PI).abs() < 1e-12
            && lat1.abs() > 1e-6);
    if pole_pair {
        let (a, _) = vincenty_ab_ellipsoid(1.0, ellipsoid.a, ellipsoid.b);
        let dist = ellipsoid.b * a * core::f64::consts::PI;
        let (fwd, rev) = if lat1 > 0.0 {
            (180.0, 0.0)
        } else {
            (0.0, 180.0)
        };
        return Ok(VincentyInverse {
            distance_m: dist,
            forward_azimuth_deg: fwd,
            reverse_azimuth_deg: rev,
        });
    }
    if lat1.abs() < 1e-12 && lat2.abs() < 1e-12 {
        if (dlon.abs() - core::f64::consts::PI).abs() < 1e-12 {
            return Err(VincentyError::NoConvergence);
        }
        let fwd = if dlon >= 0.0 { 90.0 } else { 270.0 };
        let rev = if dlon >= 0.0 { 270.0 } else { 90.0 };
        return Ok(VincentyInverse {
            distance_m: ellipsoid.a * dlon.abs(),
            forward_azimuth_deg: fwd,
            reverse_azimuth_deg: rev,
        });
    }
    let u1 = ((1.0 - ellipsoid.f) * lat1.to_radians().tan()).atan();
    let u2 = ((1.0 - ellipsoid.f) * lat2.to_radians().tan()).atan();
    let (s_u1, c_u1, s_u2, c_u2) = (u1.sin(), u1.cos(), u2.sin(), u2.cos());
    let l = dlon;
    let (sig, _, c2_a, c_2sm, lam) =
        solve_inverse_loop_ellipsoid(s_u1, c_u1, s_u2, c_u2, l, ellipsoid.f)?;
    Ok(compute_inverse_result_ellipsoid(
        sig,
        c2_a,
        c_2sm,
        lam,
        s_u1,
        c_u1,
        s_u2,
        c_u2,
        ellipsoid.a,
        ellipsoid.b,
    ))
}

/// Computes geodesic distance and azimuths between two points on WGS84.
pub fn vincenty_inverse(
    lat1: f64,
    lon1: f64,
    lat2: f64,
    lon2: f64,
) -> Result<VincentyInverse, VincentyError> {
    vincenty_inverse_ellipsoid(lat1, lon1, lat2, lon2, &Ellipsoid::WGS84)
}

/// Computes destination coordinates and reverse azimuth on WGS84.
pub fn vincenty_direct(
    lat1: f64,
    lon1: f64,
    bearing1_deg: f64,
    distance_m: f64,
) -> Result<VincentyDirect, VincentyError> {
    vincenty_direct_ellipsoid(lat1, lon1, bearing1_deg, distance_m, &Ellipsoid::WGS84)
}

/// Computes intermediate coordinates along the geodesic arc between two points.
pub fn vincenty_intermediate_points(
    lat1: f64,
    lon1: f64,
    lat2: f64,
    lon2: f64,
    count: usize,
) -> Result<Vec<(f64, f64)>, VincentyError> {
    if count == 0 {
        return Ok(Vec::new());
    }
    let inv = vincenty_inverse(lat1, lon1, lat2, lon2)?;
    let mut points = Vec::with_capacity(count);
    for i in 1..=count {
        let frac = i as f64 / (count + 1) as f64;
        let dist = inv.distance_m * frac;
        let direct = vincenty_direct(lat1, lon1, inv.forward_azimuth_deg, dist)?;
        points.push((direct.lat_deg, direct.lon_deg));
    }
    Ok(points)
}

/// Generates a dense geodesic waypoint path sampled at a given step distance.
pub fn vincenty_geodesic_path(
    lat1: f64,
    lon1: f64,
    lat2: f64,
    lon2: f64,
    step_m: f64,
) -> Result<Vec<(f64, f64)>, VincentyError> {
    if !step_m.is_finite() || step_m <= 0.0 {
        return Err(VincentyError::NegativeDistance);
    }
    let inv = vincenty_inverse(lat1, lon1, lat2, lon2)?;
    let steps_f = (inv.distance_m / step_m).ceil();
    if !steps_f.is_finite() || steps_f > crate::rhumb_line::MAX_SAMPLE_COUNT as f64 {
        return Err(VincentyError::NegativeDistance);
    }
    let mut path = vec![(lat1, lon1)];
    if inv.distance_m > 1e-6 {
        let mut d = step_m;
        while d < inv.distance_m {
            let wp = vincenty_direct(lat1, lon1, inv.forward_azimuth_deg, d)?;
            path.push((wp.lat_deg, wp.lon_deg));
            d += step_m;
        }
        path.push((lat2, lon2));
    }
    Ok(path)
}
