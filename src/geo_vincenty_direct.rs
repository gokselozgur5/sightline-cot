//! Vincenty direct geodesic algorithm on reference ellipsoids.

pub use crate::ellipsoid::{Ellipsoid, WGS84};
pub use crate::geo_vincenty::{VincentyDirect, VincentyError};

fn validate_direct_inputs(
    lat: f64,
    lon: f64,
    bearing: f64,
    dist: f64,
) -> Result<(), VincentyError> {
    if !lat.is_finite() || !lon.is_finite() || !bearing.is_finite() || !dist.is_finite() {
        return Err(VincentyError::NonFiniteInput);
    }
    if !(-90.0..=90.0).contains(&lat) {
        return Err(VincentyError::InvalidLatitude);
    }
    if !(-180.0..=180.0).contains(&lon) {
        return Err(VincentyError::InvalidLongitude);
    }
    if dist < 0.0 {
        return Err(VincentyError::NegativeDistance);
    }
    Ok(())
}

fn vincenty_ab(u2: f64) -> (f64, f64) {
    let a = 1.0 + u2 / 16384.0 * (4096.0 + u2 * (-768.0 + u2 * (320.0 - 175.0 * u2)));
    let b = u2 / 1024.0 * (256.0 + u2 * (-128.0 + u2 * (74.0 - 47.0 * u2)));
    (a, b)
}

fn delta_sigma(b: f64, sin_sig: f64, cos_sig: f64, cos_2sm: f64) -> f64 {
    let cos2_2sm = cos_2sm * cos_2sm;
    b * sin_sig
        * (cos_2sm
            + b / 4.0
                * (cos_sig * (-1.0 + 2.0 * cos2_2sm)
                    - b / 6.0
                        * cos_2sm
                        * (-3.0 + 4.0 * sin_sig * sin_sig)
                        * (-3.0 + 4.0 * cos2_2sm)))
}

fn solve_direct_sigma(
    b_semi: f64,
    s: f64,
    a_coeff: f64,
    sig1: f64,
    b_coeff: f64,
) -> (f64, f64) {
    let mut sigma = s / (b_semi * a_coeff);
    let mut cos_2sm = 0.0;
    for _ in 0..200 {
        let two_sm = 2.0 * sig1 + sigma;
        cos_2sm = two_sm.cos();
        let d_sig = delta_sigma(b_coeff, sigma.sin(), sigma.cos(), cos_2sm);
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
    lon1: f64, s_u1: f64, c_u1: f64, s_alp: f64, c2_alp: f64,
    alp1: f64, sig: f64, c_2sm: f64, f: f64,
) -> VincentyDirect {
    let (s_s, c_s) = (sig.sin(), sig.cos());
    let x = s_u1 * s_s - c_u1 * c_s * alp1.cos();
    let num = s_u1 * c_s + c_u1 * s_s * alp1.cos();
    let den = (1.0 - f) * (s_alp * s_alp + x * x).sqrt();
    let lat2 = num.atan2(den).to_degrees();
    let lam = (s_s * alp1.sin()).atan2(c_u1 * c_s - s_u1 * s_s * alp1.cos());
    let c = f / 16.0 * c2_alp * (4.0 + f * (4.0 - 3.0 * c2_alp));
    let t = c_2sm + c * c_s * (-1.0 + 2.0 * c_2sm * c_2sm);
    let omega = (1.0 - c) * f * s_alp * (sig + c * s_s * t);
    let lon2 = (lon1 + (lam - omega).to_degrees() + 540.0).rem_euclid(360.0) - 180.0;
    let rev_az = (s_alp.atan2(-x).to_degrees() + 360.0).rem_euclid(360.0);
    VincentyDirect { lat_deg: lat2, lon_deg: lon2, reverse_azimuth_deg: rev_az }
}
