//! Geodetic polyline length and fixed-spacing resampling.

pub const WGS84_EARTH_RADIUS: f64 = 6_371_008.8;

/// Haversine distance in metres on a sphere of given radius.
pub fn haversine_distance_radius(p1: [f64; 2], p2: [f64; 2], radius: f64) -> f64 {
    let (lat1, lat2) = (p1[0].to_radians(), p2[0].to_radians());
    let (dlat, dlon) = ((p2[0] - p1[0]).to_radians(), (p2[1] - p1[1]).to_radians());
    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    2.0 * radius * a.clamp(0.0, 1.0).sqrt().asin()
}

/// Haversine distance in metres using WGS84 mean earth radius.
pub fn haversine_distance(p1: [f64; 2], p2: [f64; 2]) -> f64 {
    haversine_distance_radius(p1, p2, WGS84_EARTH_RADIUS)
}

/// Great-circle intermediate point between `p1` and `p2` at fraction `f` in [0, 1].
pub fn great_circle_intermediate(p1: [f64; 2], p2: [f64; 2], f: f64) -> [f64; 2] {
    if f <= 0.0 {
        return p1;
    }
    if f >= 1.0 {
        return p2;
    }
    let (lat1, lat2) = (p1[0].to_radians(), p2[0].to_radians());
    let (lon1, lon2) = (p1[1].to_radians(), p2[1].to_radians());
    let (dlat, dlon) = (lat2 - lat1, lon2 - lon1);
    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let delta = 2.0 * a.clamp(0.0, 1.0).sqrt().asin();
    if delta.sin().abs() < 1e-15 {
        return p1;
    }
    let (sa, sb) = (
        ((1.0 - f) * delta).sin() / delta.sin(),
        (f * delta).sin() / delta.sin(),
    );
    let x = sa * lat1.cos() * lon1.cos() + sb * lat2.cos() * lon2.cos();
    let y = sa * lat1.cos() * lon1.sin() + sb * lat2.cos() * lon2.sin();
    let z = sa * lat1.sin() + sb * lat2.sin();
    [
        z.atan2((x * x + y * y).sqrt()).to_degrees(),
        y.atan2(x).to_degrees(),
    ]
}

/// Total length in metres of a polyline with a custom sphere radius.
pub fn polyline_length_radius(points: &[[f64; 2]], radius: f64) -> f64 {
    if points.len() < 2 {
        return 0.0;
    }
    points
        .windows(2)
        .map(|w| haversine_distance_radius(w[0], w[1], radius))
        .sum()
}

/// Total length in metres of a polyline with WGS84 mean earth radius.
pub fn polyline_length(points: &[[f64; 2]]) -> f64 {
    polyline_length_radius(points, WGS84_EARTH_RADIUS)
}

/// Resamples a polyline at fixed spacing `step_m` using a custom radius.
pub fn polyline_resample_radius(points: &[[f64; 2]], step_m: f64, radius: f64) -> Vec<[f64; 2]> {
    if points.len() < 2 || step_m <= 0.0 || !step_m.is_finite() {
        return points.to_vec();
    }
    let total_len = polyline_length_radius(points, radius);
    if total_len == 0.0 {
        return vec![points[0]];
    }
    let segs: Vec<f64> = points
        .windows(2)
        .map(|w| haversine_distance_radius(w[0], w[1], radius))
        .collect();
    let (mut res, mut target, mut idx, mut start) = (Vec::new(), 0.0, 0, 0.0);
    while target <= total_len + step_m * 1e-9 {
        let clamped = target.min(total_len);
        while idx + 1 < segs.len() && clamped > start + segs[idx] {
            start += segs[idx];
            idx += 1;
        }
        let pt = if segs[idx] <= 1e-12 {
            points[idx]
        } else {
            let f = ((clamped - start) / segs[idx]).clamp(0.0, 1.0);
            great_circle_intermediate(points[idx], points[idx + 1], f)
        };
        res.push(pt);
        target += step_m;
    }
    res
}

/// Resamples a polyline at fixed spacing `step_m` using WGS84 radius.
pub fn polyline_resample(points: &[[f64; 2]], step_m: f64) -> Vec<[f64; 2]> {
    polyline_resample_radius(points, step_m, WGS84_EARTH_RADIUS)
}
