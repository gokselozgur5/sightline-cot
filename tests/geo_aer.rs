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
