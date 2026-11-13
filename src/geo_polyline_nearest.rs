//! Geodesic nearest point on a polyline or segment on the sphere.

/// Default Earth mean radius in meters.
pub const DEFAULT_EARTH_RADIUS_M: f64 = 6_371_000.0;

/// Result of finding the nearest point on a polyline or segment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PolylineNearest {
    /// Nearest point on the polyline as [latitude_deg, longitude_deg].
    pub point: [f64; 2],
    /// Geodesic distance in meters from the query point to the nearest point.
    pub distance_m: f64,
    /// 0-based index of the polyline segment containing the nearest point.
    pub segment_index: usize,
    /// Fractional position along the segment in [0.0, 1.0].
    pub fraction: f64,
}

fn to_unit_vec(lat_deg: f64, lon_deg: f64) -> [f64; 3] {
    let (phi, lam) = (lat_deg.to_radians(), lon_deg.to_radians());
    let c = phi.cos();
    [c * lam.cos(), c * lam.sin(), phi.sin()]
}

fn to_lat_lon(v: [f64; 3]) -> [f64; 2] {
    let hyp = (v[0] * v[0] + v[1] * v[1]).sqrt();
    [v[2].atan2(hyp).to_degrees(), v[1].atan2(v[0]).to_degrees()]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn angle_rad(a: [f64; 3], b: [f64; 3]) -> f64 {
    dot(a, b).clamp(-1.0, 1.0).acos()
}

fn segment_normal(va: [f64; 3], vb: [f64; 3]) -> (Option<[f64; 3]>, f64) {
    let seg_len = angle_rad(va, vb);
    let n = cross(va, vb);
    let n_sq = dot(n, n);
    if seg_len < 1e-12 || n_sq < 1e-24 {
        (None, seg_len)
    } else {
        let inv = 1.0 / n_sq.sqrt();
        (Some([n[0] * inv, n[1] * inv, n[2] * inv]), seg_len)
    }
}

fn project_to_plane(vq: [f64; 3], n_hat: [f64; 3]) -> Option<[f64; 3]> {
    let qn = dot(vq, n_hat);
    let vp = [
        vq[0] - qn * n_hat[0],
        vq[1] - qn * n_hat[1],
        vq[2] - qn * n_hat[2],
    ];
    let v_sq = dot(vp, vp);
    if v_sq < 1e-24 {
        return None;
    }
    let inv = 1.0 / v_sq.sqrt();
    let mut f = [vp[0] * inv, vp[1] * inv, vp[2] * inv];
    if dot(vq, f) < 0.0 {
        f = [-f[0], -f[1], -f[2]];
    }
    Some(f)
}

fn endpoint_res(pt: [f64; 2], dist: f64, frac: f64) -> PolylineNearest {
    PolylineNearest {
        point: pt,
        distance_m: dist,
        segment_index: 0,
        fraction: frac,
    }
}

fn solve_foot_point(
    va: [f64; 3],
    vb: [f64; 3],
    vq: [f64; 3],
    n_hat: [f64; 3],
    seg_len: f64,
    radius_m: f64,
) -> Option<PolylineNearest> {
    let f_hat = project_to_plane(vq, n_hat)?;
    let d_a = angle_rad(va, vq) * radius_m;
    let d_b = angle_rad(vb, vq) * radius_m;
    if dot(cross(va, f_hat), n_hat) < 0.0 {
        Some(endpoint_res(to_lat_lon(va), d_a, 0.0))
    } else if dot(cross(f_hat, vb), n_hat) < 0.0 {
        Some(endpoint_res(to_lat_lon(vb), d_b, 1.0))
    } else {
        let frac = (angle_rad(va, f_hat) / seg_len).clamp(0.0, 1.0);
        Some(PolylineNearest {
            point: to_lat_lon(f_hat),
            distance_m: angle_rad(f_hat, vq) * radius_m,
            segment_index: 0,
            fraction: frac,
        })
    }
}

/// Find nearest point on a great-circle segment with custom sphere radius.
pub fn nearest_point_on_segment_with_radius(
    a: [f64; 2],
    b: [f64; 2],
    query: [f64; 2],
    radius_m: f64,
) -> PolylineNearest {
    let (va, vb, vq) = (
        to_unit_vec(a[0], a[1]),
        to_unit_vec(b[0], b[1]),
        to_unit_vec(query[0], query[1]),
    );
    let (n_opt, seg_len) = segment_normal(va, vb);
    let n_hat = match n_opt {
        Some(nh) => nh,
        None => return endpoint_res(a, angle_rad(va, vq) * radius_m, 0.0),
    };
    solve_foot_point(va, vb, vq, n_hat, seg_len, radius_m)
        .unwrap_or_else(|| endpoint_res(a, std::f64::consts::FRAC_PI_2 * radius_m, 0.0))
}

/// Find nearest point on a great-circle segment using default Earth radius.
pub fn nearest_point_on_segment(a: [f64; 2], b: [f64; 2], query: [f64; 2]) -> PolylineNearest {
    nearest_point_on_segment_with_radius(a, b, query, DEFAULT_EARTH_RADIUS_M)
}

/// Find nearest point on a polyline with custom sphere radius.
pub fn nearest_point_on_polyline_with_radius(
    polyline: &[[f64; 2]],
    query: [f64; 2],
    radius_m: f64,
) -> Option<PolylineNearest> {
    if polyline.is_empty() {
        return None;
    }
    if polyline.len() == 1 {
        let p0 = polyline[0];
        let v0 = to_unit_vec(p0[0], p0[1]);
        let vq = to_unit_vec(query[0], query[1]);
        return Some(endpoint_res(p0, angle_rad(v0, vq) * radius_m, 0.0));
    }
    let mut best: Option<PolylineNearest> = None;
    for i in 0..(polyline.len() - 1) {
        let mut cand =
            nearest_point_on_segment_with_radius(polyline[i], polyline[i + 1], query, radius_m);
        cand.segment_index = i;
        match &best {
            None => best = Some(cand),
            Some(curr) if cand.distance_m < curr.distance_m => best = Some(cand),
            _ => {}
        }
    }
    best
}

/// Find nearest point on a polyline using default Earth radius.
pub fn nearest_point_on_polyline(
    polyline: &[[f64; 2]],
    query: [f64; 2],
) -> Option<PolylineNearest> {
    nearest_point_on_polyline_with_radius(polyline, query, DEFAULT_EARTH_RADIUS_M)
}
