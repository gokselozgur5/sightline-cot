    // At a pole every longitude names the same point, so a datum shift must still
    // return a finite longitude inside [-180, 180].
    use sightline_cot::geo_molodensky::MolodenskyShift;

    #[test]
    fn shift_at_the_pole_returns_a_valid_longitude() {
        let shift = MolodenskyShift {
            a: 6_378_137.0,
            f: 1.0 / 298.257_223_563,
            da: 69.4,
            df: 1.0 / 294.978_698_2 - 1.0 / 298.257_223_563,
            dxyz: [8.0, -160.0, -176.0],
        };
        let (lat, lon, _) = shift.apply(90.0, 0.0, 0.0);
        assert!((lat - 90.0).abs() < 1e-3);
        assert!((-180.0..=180.0).contains(&lon), "longitude {lon}");
    }
