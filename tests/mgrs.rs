//! Contract tests for MGRS decoding and projection consistency.

use sightline_cot::mgrs::parse_mgrs_10figure;
use sightline_cot::mgrs_decode::{decode_mgrs, decode_mgrs_utm};
use sightline_cot::mgrs_parse::{parse_mgrs_to_utm, parse_mgrs_to_wgs84};

#[test]
fn test_repro_mgrs_decoders_agree_and_spaced() {
    let s = "18TWK8536028236";
    let u_parse = parse_mgrs_to_utm(s).expect("parse_mgrs_to_utm");
    let u_decode = decode_mgrs_utm(s).expect("decode_mgrs_utm");
    let fig = parse_mgrs_10figure(s).expect("parse_mgrs_10figure");
    let u_fig = fig.to_utm().expect("fig.to_utm");

    assert_eq!(u_parse.zone, u_decode.zone);
    assert_eq!(u_parse.hemisphere, u_decode.hemisphere);
    assert!((u_parse.easting - u_decode.easting).abs() < 1e-6);
    assert!((u_parse.northing - u_decode.northing).abs() < 1e-6);

    assert_eq!(u_parse.zone, u_fig.zone);
    assert_eq!(u_parse.hemisphere, u_fig.hemisphere);
    assert!((u_parse.easting - u_fig.easting).abs() < 1e-6);
    assert!((u_parse.northing - u_fig.northing).abs() < 1e-6);

    let spaced = "18T WK 85360 28236";
    let u_spaced_dec = decode_mgrs_utm(spaced).expect("decode_mgrs_utm spaced");
    assert!((u_parse.easting - u_spaced_dec.easting).abs() < 1e-6);
    assert!((u_parse.northing - u_spaced_dec.northing).abs() < 1e-6);

    let (w_parse_lat, w_parse_lon) = parse_mgrs_to_wgs84(s).expect("parse wgs84");
    let (w_dec_lat, w_dec_lon) = decode_mgrs(s).expect("decode wgs84");
    let (w_fig_lat, w_fig_lon) = fig.to_lat_lon().expect("fig to_lat_lon");

    assert!((w_parse_lat - w_dec_lat).abs() < 1e-7);
    assert!((w_parse_lon - w_dec_lon).abs() < 1e-7);
    assert!((w_parse_lat - w_fig_lat).abs() < 1e-7);
    assert!((w_parse_lon - w_fig_lon).abs() < 1e-7);
}
