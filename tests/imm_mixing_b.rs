use sightline_cot::imm::{ImmConfig, ImmFilter};

#[test]
fn update_applies_predicted_mode_probabilities() {
    let mut imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    imm.probs = [1.0, 0.0, 0.0];
    // Identical sub-filters give equal likelihoods, so the posterior
    // must equal c_bar, which is the first transition row here.
    imm.update([0.0; 3]);
    let expected = imm.trans_mat[0];
    for (got, want) in imm.probs.iter().zip(expected) {
        assert!((got - want).abs() < 1e-12, "{:?}", imm.probs);
    }
}
