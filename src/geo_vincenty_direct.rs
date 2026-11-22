//! Vincenty direct geodesic algorithm on reference ellipsoids.

pub use crate::ellipsoid::{Ellipsoid, WGS84};
use crate::geo_vincenty::{
    validate_point, vincenty_ab_ellipsoid, vincenty_c_ellipsoid, vincenty_delta_sigma,
};
pub use crate::geo_vincenty::{VincentyDirect, VincentyError};

fn validate_direct_inputs(
    lat: f64,
    lon: f64,
    bearing: f64,
    dist: f64,
) -> Result<(), VincentyError> {
    validate_point(lat, lon)?;
    if !bearing.is_finite() || !dist.is_finite() {
        return Err(VincentyError::NonFiniteInput);
    }
    if dist < 0.0 {
        return Err(VincentyError::NegativeDistance);
    }
    Ok(())
}

fn solve_direct_sigma(b_semi: f64, s: f64, a_coeff: f64, sig1: f64, b_coeff: f64) -> (f64, f64) {
    let mut sigma = s / (b_semi * a_coeff);
    let mut cos_2sm = 0.0;
    for _ in 0..200 {
        let two_sm = 2.0 * sig1 + sigma;
        cos_2sm = two_sm.cos();
        let d_sig = vincenty_delta_sigma(b_coeff, sigma.sin(), sigma.cos(), cos_2sm);
        let next = s / (b_semi * a_coeff) + d_sig;
        if (next - sigma).abs() < 1e-12 {
            return (next, cos_2sm);
        }
        sigma = next;
    }
    (sigma, cos_2sm)
}

#[allow(clippy::too_many_arguments)]
fn compute_destination(
    lon1: f64,
    s_u1: f64,
    c_u1: f64,
    s_alp: f64,
    c2_alp: f64,
    alp1: f64,
    sig: f64,
    c_2sm: f64,
    f: f64,
) -> VincentyDirect {
    let (s_s, c_s) = (sig.sin(), sig.cos());
    let x = s_u1 * s_s - c_u1 * c_s * alp1.cos();
    let num = s_u1 * c_s + c_u1 * s_s * alp1.cos();
    let den = (1.0 - f) * (s_alp * s_alp + x * x).sqrt();
    let lat2 = num.atan2(den).to_degrees();
    let lam = (s_s * alp1.sin()).atan2(c_u1 * c_s - s_u1 * s_s * alp1.cos());
    let c = vincenty_c_ellipsoid(c2_alp, f);
    let t = c_2sm + c * c_s * (-1.0 + 2.0 * c_2sm * c_2sm);
    let omega = (1.0 - c) * f * s_alp * (sig + c * s_s * t);
    let lon2 = (lon1 + (lam - omega).to_degrees() + 540.0).rem_euclid(360.0) - 180.0;
    let rev_az = (s_alp.atan2(-x).to_degrees() + 540.0).rem_euclid(360.0);
    VincentyDirect {
        lat_deg: lat2,
        lon_deg: lon2,
        reverse_azimuth_deg: rev_az,
    }
}

/// Computes destination coordinates on a reference ellipsoid using Vincenty direct formula.
pub fn vincenty_direct_ellipsoid(
    lat1: f64,
    lon1: f64,
    bearing1_deg: f64,
    distance_m: f64,
    ellipsoid: &Ellipsoid,
) -> Result<VincentyDirect, VincentyError> {
    validate_direct_inputs(lat1, lon1, bearing1_deg, distance_m)?;
    let alp1 = bearing1_deg.to_radians();
    let tan_u1 = (1.0 - ellipsoid.f) * lat1.to_radians().tan();
    let cos_u1 = 1.0 / (1.0 + tan_u1 * tan_u1).sqrt();
    let (sin_u1, sig1) = (tan_u1 * cos_u1, tan_u1.atan2(alp1.cos()));
    let sin_alp = cos_u1 * alp1.sin();
    let cos2_alp = 1.0 - sin_alp * sin_alp;
    let (a, b) = vincenty_ab_ellipsoid(cos2_alp, ellipsoid.a, ellipsoid.b);
    let (sig, cos_2sm) = solve_direct_sigma(ellipsoid.b, distance_m, a, sig1, b);
    Ok(compute_destination(
        lon1,
        sin_u1,
        cos_u1,
        sin_alp,
        cos2_alp,
        alp1,
        sig,
        cos_2sm,
        ellipsoid.f,
    ))
}

/// Computes destination coordinates on WGS84 using Vincenty direct formula.
pub fn vincenty_direct(
    lat1: f64,
    lon1: f64,
    bearing1_deg: f64,
    distance_m: f64,
) -> Result<VincentyDirect, VincentyError> {
    vincenty_direct_ellipsoid(lat1, lon1, bearing1_deg, distance_m, &Ellipsoid::WGS84)
}
