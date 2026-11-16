use sightline_cot::{
    aer_to_geodetic, geodetic_to_aer, AerCoord, SezCoord,
};

#[test]
fn test_integration_aer_cardinal_and_zenith() {
    let obs_lat = 0.0;
    let obs_lon = 0.0;
    let obs_hae = 0.0;
    let aer_up = AerCoord::new(0.0, 90.0, 1000.0).unwrap();
    let (tgt_lat, tgt_lon, tgt_hae) = aer_to_geodetic(aer_up, obs_lat, obs_lon, obs_hae).unwrap();
    assert!(tgt_lat.abs() < 1e-5);
    assert!(tgt_lon.abs() < 1e-5);
    assert!((tgt_hae - 1000.0).abs() < 1e-2);

    let aer_back = geodetic_to_aer(tgt_lat, tgt_lon, tgt_hae, obs_lat, obs_lon, obs_hae).unwrap();
    assert!((aer_back.elevation_deg - 90.0).abs() < 1e-4);
    assert!((aer_back.range_m - 1000.0).abs() < 1e-2);
}

#[test]
fn test_integration_aer_errors_and_zero_range() {
    let obs_lat = 45.0;
    let obs_lon = 9.0;
    let obs_hae = 100.0;

    let res = geodetic_to_aer(95.0, 9.0, 100.0, obs_lat, obs_lon, obs_hae);
    assert!(res.is_err());

    let res_obs = geodetic_to_aer(45.0, 9.0, 100.0, -95.0, obs_lon, obs_hae);
    assert!(res_obs.is_err());

    let zero_aer = geodetic_to_aer(obs_lat, obs_lon, obs_hae, obs_lat, obs_lon, obs_hae).unwrap();
    assert!(zero_aer.range_m < 1e-6);

    let enu = [5.0, -12.0, 100.0];
    let sez = sightline_cot::enu_to_sez(enu);
    assert_eq!(sightline_cot::sez_to_enu(sez), enu);
}
