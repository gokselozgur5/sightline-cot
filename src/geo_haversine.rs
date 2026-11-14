use std::f64;

/// Computes the great-circle distance and initial bearing between two points on a sphere.
///
/// # Arguments
///
/// * `lat1` - Latitude of the first point in degrees.
* `lon1` - Longitude of the first point in degrees.
* `lat2` - Latitude of the second point in degrees.
* `lon2` - Longitude of the second point in degrees.
*
/// # Returns
///
/// A tuple containing the great-circle distance in kilometers and the initial bearing in degrees.
fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> (f64, f64) {
    let earth_radius = 6371.0; // Earth's radius in kilometers

    let dlat = lat2.to_radians() - lat1.to_radians();
    let dlon = lon2.to_radians() - lon1.to_radians();

    let a = (dlat / 2.0).sin().powi(2) + (dlon / 2.0).sin().powi(2) * earth_radius.cos().powi(2);
    let c = 2.0 * earth_radius.atan2((a.sqrt()), (1.0 - a));

    let distance = c * earth_radius;
    let bearing = ((lon2.to_radians() - lon1.to_radians()) + f64::consts::PI) % (f64::consts::PI * 2.0);

    (distance, bearing)
}
