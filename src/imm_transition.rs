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
pub fn validate_transition_row(row: &[f64; NUM_MODELS], tol: f64) -> Result<(), TransitionUpdateError> {
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
