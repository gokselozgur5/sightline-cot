use sightline_cot::imm::{ImmConfig, ImmFilter};
use sightline_cot::imm_transition::{
    update_transition_matrix, validate_transition_row, TransitionUpdateError,
};

#[test]
fn test_validate_transition_row_nan_and_negative_tol() {
    let row = [0.5, 0.5, 0.5];
    assert_eq!(
        validate_transition_row(&row, f64::NAN),
        Err(TransitionUpdateError::NonFinite)
    );
    assert_eq!(
        validate_transition_row(&row, f64::INFINITY),
        Err(TransitionUpdateError::NonFinite)
    );
    assert_eq!(
        validate_transition_row(&row, f64::NEG_INFINITY),
        Err(TransitionUpdateError::NonFinite)
    );
    assert_eq!(
        validate_transition_row(&[0.8, 0.1, 0.1], -1e-6),
        Err(TransitionUpdateError::RowSumNotOne)
    );
}

#[test]
fn test_update_transition_matrix_nan_tol_rejected() {
    let mut imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    let non_stochastic = [[0.5, 0.5, 0.5]; 3];
    assert_eq!(
        update_transition_matrix(&mut imm, non_stochastic, f64::NAN),
        Err(TransitionUpdateError::NonFinite)
    );
    assert_eq!(
        update_transition_matrix(&mut imm, non_stochastic, f64::INFINITY),
        Err(TransitionUpdateError::NonFinite)
    );
}

#[test]
fn test_trans_mat_is_valid_non_finite_tol() {
    let imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    assert!(!imm.trans_mat_is_valid(f64::NAN));
    assert!(!imm.trans_mat_is_valid(f64::INFINITY));
    assert!(!imm.trans_mat_is_valid(-1e-6));
}
