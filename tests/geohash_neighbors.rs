use sightline_cot::geohash_neighbors::{geohash_neighbor, geohash_neighbors, GeohashDirection};

#[test]
fn test_geohash_individual_directions() {
    assert_eq!(
        geohash_neighbor("ezs42", GeohashDirection::North).unwrap(),
        "ezs48"
    );
    assert_eq!(
        geohash_neighbor("ezs42", GeohashDirection::South).unwrap(),
        "ezs40"
    );
    assert_eq!(
        geohash_neighbor("ezs42", GeohashDirection::East).unwrap(),
        "ezs43"
    );
    assert_eq!(
        geohash_neighbor("ezs42", GeohashDirection::West).unwrap(),
        "ezefr"
    );
    assert_eq!(
        geohash_neighbor("ezs42", GeohashDirection::NorthEast).unwrap(),
        "ezs49"
    );
    assert_eq!(
        geohash_neighbor("ezs42", GeohashDirection::NorthWest).unwrap(),
        "ezefx"
    );
    assert_eq!(
        geohash_neighbor("ezs42", GeohashDirection::SouthEast).unwrap(),
        "ezs41"
    );
    assert_eq!(
        geohash_neighbor("ezs42", GeohashDirection::SouthWest).unwrap(),
        "ezefp"
    );
}

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

#[test]
fn test_geohash_errors() {
    use sightline_cot::geohash_neighbors::GeohashError;
    assert_eq!(geohash_neighbors(""), Err(GeohashError::EmptyString));
    assert_eq!(
        geohash_neighbors("ezs4a"),
        Err(GeohashError::InvalidCharacter('a'))
    );
    assert_eq!(
        geohash_neighbor("ezs4i", GeohashDirection::North),
        Err(GeohashError::InvalidCharacter('i'))
    );
}

#[test]
fn test_geohash_single_char_and_case() {
    let neighbors = geohash_neighbors("u").unwrap();
    assert_eq!(neighbors.north, "h");
    assert_eq!(neighbors.south, "s");
    assert_eq!(neighbors.east, "v");
    assert_eq!(neighbors.west, "g");

    let upper = geohash_neighbors("EZS42").unwrap();
    assert_eq!(upper.north, "ezs48");
    assert_eq!(upper.east, "ezs43");
}
