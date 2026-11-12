use sightline_cot::imm::{ImmConfig, ImmFilter, MODEL_CV};

#[test]
fn mix_adds_spread_of_means_to_covariance() {
    let mut imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    imm.probs = [0.5, 0.5, 0.0];
    imm.filters[1].state[3] = 10.0;
    imm.mix();
    // mu = [0.85, 0.15], mean = 1.5, spread = 0.85 * 2.25 + 0.15 * 72.25.
    let expected = 10.0 + 12.75;
    let got = imm.filters[MODEL_CV].cov[3][3];
    assert!((got - expected).abs() < 1e-9, "got {got}");
}
