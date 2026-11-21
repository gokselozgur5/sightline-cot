use sightline_cot::imm::{ImmConfig, ImmFilter};
use sightline_cot::imm_transition::{
    adaptive_transition_matrix, update_transition_matrix, validate_transition_row,
    TransitionUpdateError,
};

#[test]
fn test_validate_transition_row_valid() {
    let row = [0.8, 0.1, 0.1];
    assert!(validate_transition_row(&row, 1e-6).is_ok());
    assert_eq!(validate_transition_row(&row, 1e-6), Ok(()));
}

#[test]
fn test_validate_transition_row_errors() {
    assert_eq!(
        validate_transition_row(&[f64::NAN, 0.5, 0.5], 1e-6),
        Err(TransitionUpdateError::NonFinite)
    );
    assert_eq!(
        validate_transition_row(&[-0.1, 0.6, 0.5], 1e-6),
        Err(TransitionUpdateError::InvalidProbability)
    );
    assert_eq!(
        validate_transition_row(&[0.5, 0.5, 0.5], 1e-6),
        Err(TransitionUpdateError::RowSumNotOne)
    );
}

#[test]
fn test_update_transition_matrix_valid_and_invalid() {
    let mut imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    let valid_mat = [[0.7, 0.2, 0.1], [0.1, 0.8, 0.1], [0.1, 0.2, 0.7]];
    assert!(update_transition_matrix(&mut imm, valid_mat, 1e-6).is_ok());
    assert_eq!(imm.trans_mat, valid_mat);

    let mut invalid_mat = valid_mat;
    invalid_mat[0][0] = 0.9;
    assert_eq!(
        update_transition_matrix(&mut imm, invalid_mat, 1e-6),
        Err(TransitionUpdateError::RowSumNotOne)
    );
    assert_eq!(imm.trans_mat, valid_mat);
}

#[test]
fn test_adaptive_transition_matrix() {
    let sojourn = [10.0, 5.0, 4.0];
    let dt = 0.5;
    let res = adaptive_transition_matrix(sojourn, dt).unwrap();
    for (i, row) in res.iter().enumerate() {
        assert!((row.iter().sum::<f64>() - 1.0).abs() < 1e-6);
        let expected_diag = 1.0 - (dt / sojourn[i]);
        assert!((row[i] - expected_diag).abs() < 1e-6);
    }
}
