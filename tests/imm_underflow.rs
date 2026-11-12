use sightline_cot::imm::{ImmConfig, ImmFilter, SubFilter};

#[test]
fn closer_innovation_scores_higher_than_farther() {
    let near = SubFilter::new([0.0; 3], 4.0, 10.0).update_pos([30.0, 0.0, 0.0], 4.0);
    let far = SubFilter::new([0.0; 3], 4.0, 10.0).update_pos([60.0, 0.0, 0.0], 4.0);
    assert!(near > far, "near {near} far {far}");
}

#[test]
fn equal_tiny_likelihoods_keep_predicted_probabilities() {
    let mut imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    let c_bar = imm.compute_c_bar();
    imm.update([40.0, 0.0, 0.0]);
    for (got, want) in imm.probs.iter().zip(c_bar) {
        assert!((got - want).abs() < 1e-9, "{:?}", imm.probs);
    }
}

#[test]
fn huge_outlier_keeps_probabilities_normalized() {
    let mut imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    imm.update([1e9, 0.0, 0.0]);
    let sum: f64 = imm.probs.iter().sum();
    assert!((sum - 1.0).abs() < 1e-9, "{:?}", imm.probs);
    assert!(imm.probs.iter().all(|p| p.is_finite()));
}
