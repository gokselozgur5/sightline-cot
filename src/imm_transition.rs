/// Error returned when transition matrix probability update is invalid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionUpdateError {
    InvalidMatrixDimensions,
    InvalidProbability,
    RowSumNotOne,
    NonFinite,
}

use crate::imm::NUM_MODELS;

/// Validates whether a row represents a valid probability distribution.
pub fn validate_transition_row(
    row: &[f64; NUM_MODELS],
    tol: f64,
) -> Result<(), TransitionUpdateError> {
    if !tol.is_finite() {
        return Err(TransitionUpdateError::NonFinite);
    }
    for &p in row.iter() {
        if !p.is_finite() {
            return Err(TransitionUpdateError::NonFinite);
        }
        if !(0.0..=1.0).contains(&p) {
            return Err(TransitionUpdateError::InvalidProbability);
        }
    }
    let sum: f64 = row.iter().sum();
    if (sum - 1.0).abs() > tol {
        return Err(TransitionUpdateError::RowSumNotOne);
    }
    Ok(())
}

use crate::imm::ImmFilter;

/// Updates the transition matrix of an IMM filter with strict validation.
pub fn update_transition_matrix(
    filter: &mut ImmFilter,
    new_mat: [[f64; NUM_MODELS]; NUM_MODELS],
    tol: f64,
) -> Result<(), TransitionUpdateError> {
    for row in &new_mat {
        validate_transition_row(row, tol)?;
    }
    filter.trans_mat = new_mat;
    Ok(())
}

/// Computes adaptive transition matrix based on sojourn times (in seconds).
pub fn adaptive_transition_matrix(
    sojourn_times_s: [f64; NUM_MODELS],
    dt: f64,
) -> Result<[[f64; NUM_MODELS]; NUM_MODELS], TransitionUpdateError> {
    if !dt.is_finite() || dt <= 0.0 {
        return Err(TransitionUpdateError::NonFinite);
    }
    let mut mat = [[0.0; NUM_MODELS]; NUM_MODELS];
    for (i, &tau) in sojourn_times_s.iter().enumerate() {
        if !tau.is_finite() || tau <= dt {
            return Err(TransitionUpdateError::InvalidProbability);
        }
        let p_ii = 1.0 - (dt / tau);
        let p_ij = (dt / tau) / ((NUM_MODELS - 1) as f64);
        for (j, cell) in mat[i].iter_mut().enumerate() {
            *cell = if i == j { p_ii } else { p_ij };
        }
        validate_transition_row(&mat[i], 1e-6)?;
    }
    Ok(mat)
}
