use geo_haversine::haversine;

#[test]
fn test_haversine() {
    let (distance, bearing) = haversine(52.2296756, 4.3517556, 41.8919444, -87.6220669);

    assert!((distance - 799.9).abs() < 0.05); // 799.9 km
    assert!((bearing - 131.8).abs() < 0.05); // 131.8 degrees

    let (distance, bearing) = haversine(40.7128, -74.0060, 34.0522, -118.2437);

    assert!((distance - 395.3).abs() < 0.05); // 395.3 km
    assert!((bearing - 106.8).abs() < 0.05); // 106.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 34.0522, -118.2437);

    assert!((distance - 192.2).abs() < 0.05); // 192.2 km
    assert!((bearing - 166.8).abs() < 0.05); // 166.8 degrees

    let (distance, bearing) = haversine(47.6062, -122.3321, 40.7128, -74.0060);
