use sightline_cot::imm::{ImmConfig, ImmFilter, SubFilter};

fn assert_psd(f: &SubFilter) {
    for i in 0..6 {
        assert!(f.cov[i][i] >= 0.0, "negative variance at {i}");
        for j in 0..6 {
            let d = (f.cov[i][j] - f.cov[j][i]).abs();
            assert!(d <= 1e-9 * (1.0 + f.cov[i][j].abs()), "asym {i}{j}");
        }
    }
    for i in 0..3 {
        let det = f.cov[i][i] * f.cov[i + 3][i + 3] - f.cov[i][i + 3].powi(2);
        assert!(det >= -1e-9, "negative minor {i}: {det}");
    }
}
