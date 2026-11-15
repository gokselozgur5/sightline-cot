use sightline_cot::geohash_neighbors::{geohash_neighbors, GeohashDirection};

#[test]
fn test_geohash_neighbors_ezs42() {
    let neighbors = geohash_neighbors("ezs42").unwrap();
    assert_eq!(neighbors.north, "ezs48");
    assert_eq!(neighbors.north_east, "ezs49");
    assert_eq!(neighbors.east, "ezs43");
    assert_eq!(neighbors.south_east, "ezs41");
    assert_eq!(neighbors.south, "ezs40");
    assert_eq!(neighbors.south_west, "ezefp");
    assert_eq!(neighbors.west, "ezefr");
    assert_eq!(neighbors.north_west, "ezefx");
}
