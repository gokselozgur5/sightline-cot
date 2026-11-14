use sightline_cot::imm::{ImmConfig, ImmFilter, NUM_MODELS};
use sightline_cot::imm_transition::{
    adaptive_transition_matrix, update_transition_matrix, validate_transition_row,
    TransitionUpdateError,
};

#[test]
fn test_validate_transition_row_valid() {
    let row = [0.8, 0.1, 0.1];
    assert!(validate_transition_row(&row, 1e-6).is_ok());
}
