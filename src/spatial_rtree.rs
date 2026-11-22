//! Geodesic polygon bounding box and spatial indexing R-Tree helper.

/// Earth's mean radius in meters according to WGS-84 spherical approximation.
pub const EARTH_RADIUS_M: f64 = 6_371_008.8;

/// Normalizes a longitude coordinate into the standard [-180.0, 180.0) range.
pub fn normalize_lon(lon: f64) -> f64 {
    crate::egm96_geoid::normalize_lon_180(lon)
}

/// Great-circle distance between two geodetic coordinates in meters.
pub fn haversine_distance_m(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    crate::geo_haversine::haversine_distance_m(
        [lat1, normalize_lon(lon1)],
        [lat2, normalize_lon(lon2)],
    )
}

/// 2D axis-aligned geodetic spatial bounding box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialEnvelope {
    pub min_lat: f64,
    pub min_lon: f64,
    pub max_lat: f64,
    pub max_lon: f64,
}

impl SpatialEnvelope {
    /// Creates a new geodetic envelope with explicit boundaries.
    pub fn new(min_lat: f64, min_lon: f64, max_lat: f64, max_lon: f64) -> Self {
        Self {
            min_lat,
            min_lon,
            max_lat,
            max_lon,
        }
    }

    /// Creates an envelope enclosing a single geodetic point.
    pub fn from_point(lat: f64, lon: f64) -> Self {
        let norm_lon = normalize_lon(lon);
        Self::new(lat, norm_lon, lat, norm_lon)
    }

    fn polygon_lon_bounds(coords: &[[f64; 2]]) -> Option<(f64, f64)> {
        let mut lons: Vec<f64> = coords.iter().map(|p| normalize_lon(p[1])).collect();
        lons.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        lons.dedup();
        let (mut min_lon, mut max_lon) = (*lons.first()?, *lons.first()?);
        if lons.len() > 1 {
            let mut max_gap = -1.0;
            for i in 0..lons.len() {
                let nxt = (i + 1) % lons.len();
                let gap = (lons[nxt] - lons[i]).rem_euclid(360.0);
                if gap > max_gap {
                    max_gap = gap;
                    min_lon = lons[nxt];
                    max_lon = lons[i];
                }
            }
        }
        Some((min_lon, max_lon))
    }

    /// Computes the bounding envelope from a slice of polygon vertices [lat, lon].
    pub fn from_geodesic_polygon(coords: &[[f64; 2]]) -> Option<Self> {
        let first = coords.first()?;
        let (mut min_lat, mut max_lat) = (first[0], first[0]);
        for pt in &coords[1..] {
            min_lat = min_lat.min(pt[0]);
            max_lat = max_lat.max(pt[0]);
        }
        let (min_lon, max_lon) = Self::polygon_lon_bounds(coords)?;
        Some(Self::new(min_lat, min_lon, max_lat, max_lon))
    }

    /// Returns true if this envelope contains the given point.
    pub fn contains_point(&self, lat: f64, lon: f64) -> bool {
        let n = normalize_lon(lon);
        let lon_ok = if self.min_lon <= self.max_lon {
            n >= self.min_lon && n <= self.max_lon
        } else {
            n >= self.min_lon || n <= self.max_lon
        };
        lat >= self.min_lat && lat <= self.max_lat && lon_ok
    }

    /// Returns true if this envelope fully encloses another envelope.
    pub fn contains_envelope(&self, other: &Self) -> bool {
        let lat_ok = self.min_lat <= other.min_lat && self.max_lat >= other.max_lat;
        let s_wrap = self.min_lon > self.max_lon;
        let o_wrap = other.min_lon > other.max_lon;
        let lon_ok = match (s_wrap, o_wrap) {
            (false, false) => self.min_lon <= other.min_lon && self.max_lon >= other.max_lon,
            (true, false) => {
                (other.min_lon >= self.min_lon && other.max_lon >= self.min_lon)
                    || (other.min_lon <= self.max_lon && other.max_lon <= self.max_lon)
            }
            (false, true) => false,
            (true, true) => other.min_lon >= self.min_lon && other.max_lon <= self.max_lon,
        };
        lat_ok && lon_ok
    }

    /// Returns true if this envelope intersects with another envelope.
    pub fn intersects(&self, other: &Self) -> bool {
        let lat_overlap = self.min_lat <= other.max_lat && self.max_lat >= other.min_lat;
        let s_wrap = self.min_lon > self.max_lon;
        let o_wrap = other.min_lon > other.max_lon;
        let lon_overlap = match (s_wrap, o_wrap) {
            (false, false) => self.min_lon <= other.max_lon && self.max_lon >= other.min_lon,
            (true, false) => other.max_lon >= self.min_lon || other.min_lon <= self.max_lon,
            (false, true) => self.max_lon >= other.min_lon || self.min_lon <= other.max_lon,
            (true, true) => true,
        };
        lat_overlap && lon_overlap
    }

    fn merge_lon_bounds(a: &Self, b: &Self) -> (f64, f64) {
        if a.contains_envelope(b) {
            return (a.min_lon, a.max_lon);
        }
        if b.contains_envelope(a) {
            return (b.min_lon, b.max_lon);
        }
        let candidates = [
            (a.min_lon, b.max_lon),
            (b.min_lon, a.max_lon),
            (a.min_lon.min(b.min_lon), a.max_lon.max(b.max_lon)),
        ];
        let mut best = (f64::MAX, a.min_lon, a.max_lon);
        for &(s, e) in &candidates {
            let env = Self::new(-90.0, s, 90.0, e);
            if env.contains_envelope(a) && env.contains_envelope(b) && env.lon_span() < best.0 {
                best = (env.lon_span(), s, e);
            }
        }
        (best.1, best.2)
    }

    /// Computes the union envelope of this and another envelope.
    pub fn merge(&self, other: &Self) -> Self {
        let (min_lon, max_lon) = Self::merge_lon_bounds(self, other);
        Self::new(
            self.min_lat.min(other.min_lat),
            min_lon,
            self.max_lat.max(other.max_lat),
            max_lon,
        )
    }

    fn lon_span(&self) -> f64 {
        if self.min_lon <= self.max_lon {
            (self.max_lon - self.min_lon).max(0.0)
        } else {
            (self.max_lon - self.min_lon).rem_euclid(360.0)
        }
    }

    /// Computes the area metric of the bounding box in degrees squared.
    pub fn area(&self) -> f64 {
        (self.max_lat - self.min_lat).max(0.0) * self.lon_span()
    }

    /// Computes the semi-perimeter margin of the bounding box.
    pub fn margin(&self) -> f64 {
        (self.max_lat - self.min_lat).max(0.0) + self.lon_span()
    }

    /// Computes center coordinate [lat, lon] of the envelope.
    pub fn center(&self) -> [f64; 2] {
        let mid_lon = normalize_lon(self.min_lon + self.lon_span() * 0.5);
        [(self.min_lat + self.max_lat) * 0.5, mid_lon]
    }

    /// Computes minimal geodesic distance in meters from point to the envelope boundary.
    fn edge_dist(lat: f64, lon: f64, min_lat: f64, max_lat: f64, edge_lon: f64) -> f64 {
        let (ql, dl) = (lat.to_radians(), normalize_lon(edge_lon - lon).to_radians());
        let mut d = haversine_distance_m(lat, lon, min_lat, edge_lon)
            .min(haversine_distance_m(lat, lon, max_lat, edge_lon));
        let c = ql.sin().atan2(ql.cos() * dl.cos()).to_degrees();
        if (-90.0..=90.0).contains(&c) && c > min_lat && c < max_lat {
            d = d.min(haversine_distance_m(lat, lon, c, edge_lon));
        }
        d
    }

    pub fn distance_to_point(&self, lat: f64, lon: f64) -> f64 {
        if self.contains_point(lat, lon) {
            return 0.0;
        }
        let dm = Self::edge_dist(lat, lon, self.min_lat, self.max_lat, self.min_lon).min(
            Self::edge_dist(lat, lon, self.min_lat, self.max_lat, self.max_lon),
        );
        let par = |plat: f64| {
            let nl = normalize_lon(lon);
            let ok = if self.min_lon <= self.max_lon {
                nl >= self.min_lon && nl <= self.max_lon
            } else {
                nl >= self.min_lon || nl <= self.max_lon
            };
            if ok {
                haversine_distance_m(lat, lon, plat, nl)
            } else {
                haversine_distance_m(lat, lon, plat, self.min_lon).min(haversine_distance_m(
                    lat,
                    lon,
                    plat,
                    self.max_lon,
                ))
            }
        };
        dm.min(par(self.min_lat)).min(par(self.max_lat))
    }
}

/// An entry held in an R-Tree node.
#[derive(Debug, Clone, PartialEq)]
pub struct RTreeEntry<T> {
    pub envelope: SpatialEnvelope,
    pub data: T,
}

impl<T> RTreeEntry<T> {
    /// Creates a new entry with the given envelope and associated data.
    pub fn new(envelope: SpatialEnvelope, data: T) -> Self {
        Self { envelope, data }
    }
}

#[derive(Debug, Clone)]
enum RTreeNode<T> {
    Leaf {
        entries: Vec<RTreeEntry<T>>,
        envelope: SpatialEnvelope,
    },
    Branch {
        children: Vec<RTreeNode<T>>,
        envelope: SpatialEnvelope,
    },
}

impl<T: Clone> RTreeNode<T> {
    fn envelope(&self) -> SpatialEnvelope {
        match self {
            Self::Leaf { envelope, .. } => *envelope,
            Self::Branch { envelope, .. } => *envelope,
        }
    }
}

/// 2D spatial indexing R-Tree for fast tactical Cursor-on-Target queries.
#[derive(Debug, Clone)]
pub struct SpatialRTree<T> {
    root: Option<RTreeNode<T>>,
    len: usize,
    max_entries: usize,
}

impl<T: Clone> Default for SpatialRTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> SpatialRTree<T> {
    /// Creates a new empty spatial R-Tree with default branch capacity (4).
    pub fn new() -> Self {
        Self::with_max_entries(4)
    }

    /// Creates a new empty spatial R-Tree with a custom maximum node capacity.
    pub fn with_max_entries(max_entries: usize) -> Self {
        let max_entries = max_entries.max(2);
        Self {
            root: None,
            len: 0,
            max_entries,
        }
    }

    /// Returns the number of indexed entries in the tree.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the R-Tree contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the global bounding envelope of all indexed elements, if any.
    pub fn root_envelope(&self) -> Option<SpatialEnvelope> {
        self.root.as_ref().map(|r| r.envelope())
    }

    /// Inserts an item with its corresponding spatial envelope into the tree.
    pub fn insert(&mut self, envelope: SpatialEnvelope, data: T) {
        let entry = RTreeEntry::new(envelope, data);
        self.len += 1;

        if self.root.is_none() {
            self.root = Some(RTreeNode::Leaf {
                entries: vec![entry],
                envelope,
            });
            return;
        }

        let mut root = self.root.take().unwrap();
        if let Some((left, right)) = Self::insert_recursive(&mut root, entry, self.max_entries) {
            let combined = left.envelope().merge(&right.envelope());
            self.root = Some(RTreeNode::Branch {
                children: vec![left, right],
                envelope: combined,
            });
        } else {
            self.root = Some(root);
        }
    }

    fn insert_recursive(
        node: &mut RTreeNode<T>,
        entry: RTreeEntry<T>,
        max_entries: usize,
    ) -> Option<(RTreeNode<T>, RTreeNode<T>)> {
        match node {
            RTreeNode::Leaf { entries, envelope } => {
                entries.push(entry);
                *envelope = envelope.merge(&entries.last().unwrap().envelope);
                if entries.len() > max_entries {
                    Some(Self::split_leaf(std::mem::take(entries)))
                } else {
                    None
                }
            }
            RTreeNode::Branch { children, envelope } => {
                let best_idx = Self::choose_best_child(children, &entry.envelope);
                if let Some((n1, n2)) =
                    Self::insert_recursive(&mut children[best_idx], entry, max_entries)
                {
                    children[best_idx] = n1;
                    children.push(n2);
                    let mut new_env = children[0].envelope();
                    for c in &children[1..] {
                        new_env = new_env.merge(&c.envelope());
                    }
                    *envelope = new_env;
                    if children.len() > max_entries {
                        Some(Self::split_branch(std::mem::take(children)))
                    } else {
                        None
                    }
                } else {
                    *envelope = envelope.merge(&children[best_idx].envelope());
                    None
                }
            }
        }
    }

    fn choose_best_child(children: &[RTreeNode<T>], env: &SpatialEnvelope) -> usize {
        let mut best_idx = 0;
        let mut min_expansion = f64::MAX;
        let mut min_area = f64::MAX;

        for (i, child) in children.iter().enumerate() {
            let cur_env = child.envelope();
            let merged = cur_env.merge(env);
            let cur_area = cur_env.area();
            let exp = merged.area() - cur_area;

            if exp < min_expansion || (exp == min_expansion && cur_area < min_area) {
                min_expansion = exp;
                min_area = cur_area;
                best_idx = i;
            }
        }

        best_idx
    }

    fn split_leaf(mut entries: Vec<RTreeEntry<T>>) -> (RTreeNode<T>, RTreeNode<T>) {
        let split_at = entries.len() / 2;
        let right_entries = entries.split_off(split_at);

        let mut left_env = entries[0].envelope;
        for e in &entries[1..] {
            left_env = left_env.merge(&e.envelope);
        }

        let mut right_env = right_entries[0].envelope;
        for e in &right_entries[1..] {
            right_env = right_env.merge(&e.envelope);
        }

        (
            RTreeNode::Leaf {
                entries,
                envelope: left_env,
            },
            RTreeNode::Leaf {
                entries: right_entries,
                envelope: right_env,
            },
        )
    }

    fn split_branch(mut children: Vec<RTreeNode<T>>) -> (RTreeNode<T>, RTreeNode<T>) {
        let split_at = children.len() / 2;
        let right_children = children.split_off(split_at);

        let mut left_env = children[0].envelope();
        for c in &children[1..] {
            left_env = left_env.merge(&c.envelope());
        }

        let mut right_env = right_children[0].envelope();
        for c in &right_children[1..] {
            right_env = right_env.merge(&c.envelope());
        }

        (
            RTreeNode::Branch {
                children,
                envelope: left_env,
            },
            RTreeNode::Branch {
                children: right_children,
                envelope: right_env,
            },
        )
    }

    /// Queries all items whose bounding envelope intersects with the search envelope.
    pub fn search_intersecting(&self, query: &SpatialEnvelope) -> Vec<&T> {
        let mut results = Vec::new();
        if let Some(ref root) = self.root {
            Self::search_node(root, query, &mut results);
        }
        results
    }

    fn search_node<'a>(node: &'a RTreeNode<T>, query: &SpatialEnvelope, results: &mut Vec<&'a T>) {
        if !node.envelope().intersects(query) {
            return;
        }

        match node {
            RTreeNode::Leaf { entries, .. } => {
                for entry in entries {
                    if entry.envelope.intersects(query) {
                        results.push(&entry.data);
                    }
                }
            }
            RTreeNode::Branch { children, .. } => {
                for child in children {
                    Self::search_node(child, query, results);
                }
            }
        }
    }

    /// Queries all items whose bounding envelope contains the given point coordinate.
    pub fn search_containing(&self, lat: f64, lon: f64) -> Vec<&T> {
        let mut results = Vec::new();
        if let Some(ref root) = self.root {
            Self::search_containing_node(root, lat, lon, &mut results);
        }
        results
    }

    fn search_containing_node<'a>(
        node: &'a RTreeNode<T>,
        lat: f64,
        lon: f64,
        results: &mut Vec<&'a T>,
    ) {
        if !node.envelope().contains_point(lat, lon) {
            return;
        }

        match node {
            RTreeNode::Leaf { entries, .. } => {
                for entry in entries {
                    if entry.envelope.contains_point(lat, lon) {
                        results.push(&entry.data);
                    }
                }
            }
            RTreeNode::Branch { children, .. } => {
                for child in children {
                    Self::search_containing_node(child, lat, lon, results);
                }
            }
        }
    }

    /// Finds the nearest indexed item to the coordinate and returns reference and distance in meters.
    pub fn nearest_neighbor(&self, lat: f64, lon: f64) -> Option<(&T, f64)> {
        let root = self.root.as_ref()?;
        let mut best: Option<(&T, f64)> = None;
        Self::nearest_recursive(root, lat, lon, &mut best);
        best
    }

    fn nearest_recursive<'a>(
        node: &'a RTreeNode<T>,
        lat: f64,
        lon: f64,
        best: &mut Option<(&'a T, f64)>,
    ) {
        let d_node = node.envelope().distance_to_point(lat, lon);
        if let Some((_, best_d)) = best {
            if d_node > *best_d {
                return;
            }
        }

        match node {
            RTreeNode::Leaf { entries, .. } => {
                for entry in entries {
                    let d = entry.envelope.distance_to_point(lat, lon);
                    match best {
                        Some((_, ref mut best_d)) => {
                            if d < *best_d {
                                *best = Some((&entry.data, d));
                            }
                        }
                        None => {
                            *best = Some((&entry.data, d));
                        }
                    }
                }
            }
            RTreeNode::Branch { children, .. } => {
                let mut sorted_children: Vec<&'a RTreeNode<T>> = children.iter().collect();
                sorted_children.sort_by(|a, b| {
                    let da = a.envelope().distance_to_point(lat, lon);
                    let db = b.envelope().distance_to_point(lat, lon);
                    da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
                });

                for child in sorted_children {
                    Self::nearest_recursive(child, lat, lon, best);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_lon() {
        assert_eq!(normalize_lon(0.0), 0.0);
        assert_eq!(normalize_lon(190.0), -170.0);
        assert_eq!(normalize_lon(-200.0), 160.0);
    }

    #[test]
    fn test_haversine_distance() {
        let d = haversine_distance_m(0.0, 0.0, 1.0, 0.0);
        assert!((d - 111195.0).abs() < 1000.0);
    }

    #[test]
    fn test_spatial_envelope_basic() {
        let env = SpatialEnvelope::new(10.0, 20.0, 30.0, 40.0);
        assert!(env.contains_point(15.0, 25.0));
        assert!(!env.contains_point(5.0, 25.0));
        assert_eq!(env.center(), [20.0, 30.0]);
        assert_eq!(env.area(), 400.0);
        assert_eq!(env.margin(), 40.0);
    }

    #[test]
    fn test_spatial_envelope_polygon() {
        let pts = [[10.0, 20.0], [15.0, 25.0], [5.0, 15.0]];
        let env = SpatialEnvelope::from_geodesic_polygon(&pts).unwrap();
        assert_eq!(env.min_lat, 5.0);
        assert_eq!(env.max_lat, 15.0);
        assert_eq!(env.min_lon, 15.0);
        assert_eq!(env.max_lon, 25.0);
    }

    #[test]
    fn test_spatial_envelope_intersects() {
        let env1 = SpatialEnvelope::new(0.0, 0.0, 10.0, 10.0);
        let env2 = SpatialEnvelope::new(5.0, 5.0, 15.0, 15.0);
        let env3 = SpatialEnvelope::new(20.0, 20.0, 30.0, 30.0);
        assert!(env1.intersects(&env2));
        assert!(!env1.intersects(&env3));
    }

    #[test]
    fn test_rtree_insert_and_queries() {
        let mut tree = SpatialRTree::with_max_entries(2);
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);

        tree.insert(SpatialEnvelope::new(0.0, 0.0, 2.0, 2.0), "A");
        tree.insert(SpatialEnvelope::new(5.0, 5.0, 7.0, 7.0), "B");
        tree.insert(SpatialEnvelope::new(10.0, 10.0, 12.0, 12.0), "C");
        tree.insert(SpatialEnvelope::new(15.0, 15.0, 17.0, 17.0), "D");

        assert_eq!(tree.len(), 4);
        assert!(!tree.is_empty());

        let q1 = SpatialEnvelope::new(1.0, 1.0, 6.0, 6.0);
        let found = tree.search_intersecting(&q1);
        assert_eq!(found.len(), 2);
        assert!(found.contains(&&"A"));
        assert!(found.contains(&&"B"));

        let containing = tree.search_containing(1.0, 1.0);
        assert_eq!(containing, vec![&"A"]);
    }

    #[test]
    fn test_rtree_nearest_neighbor() {
        let mut tree = SpatialRTree::new();
        tree.insert(SpatialEnvelope::from_point(0.0, 0.0), "Origin");
        tree.insert(SpatialEnvelope::from_point(10.0, 10.0), "Far");

        let (item, dist) = tree.nearest_neighbor(0.1, 0.1).unwrap();
        assert_eq!(*item, "Origin");
        assert!(dist < 20000.0);
    }
}
