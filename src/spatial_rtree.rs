//! Geodesic polygon bounding box and spatial indexing R-Tree helper.

/// Earth's mean radius in meters according to WGS-84 spherical approximation.
pub const EARTH_RADIUS_M: f64 = 6_371_008.8;

/// Normalizes a longitude coordinate into the standard [-180.0, 180.0) range.
pub fn normalize_lon(mut lon: f64) -> f64 {
    while lon >= 180.0 {
        lon -= 360.0;
    }
    while lon < -180.0 {
        lon += 360.0;
    }
    lon
}

/// Great-circle distance between two geodetic coordinates in meters.
pub fn haversine_distance_m(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (normalize_lon(lon2) - normalize_lon(lon1)).to_radians();
    let lat1_r = lat1.to_radians();
    let lat2_r = lat2.to_radians();

    let a = (d_lat * 0.5).sin().powi(2)
        + lat1_r.cos() * lat2_r.cos() * (d_lon * 0.5).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    EARTH_RADIUS_M * c
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

    /// Computes the bounding envelope from a slice of polygon vertices [lat, lon].
    pub fn from_geodesic_polygon(coords: &[[f64; 2]]) -> Option<Self> {
        let first = coords.first()?;
        let mut min_lat = first[0];
        let mut max_lat = first[0];
        let mut min_lon = normalize_lon(first[1]);
        let mut max_lon = min_lon;

        for pt in &coords[1..] {
            min_lat = min_lat.min(pt[0]);
            max_lat = max_lat.max(pt[0]);
            let n_lon = normalize_lon(pt[1]);
            min_lon = min_lon.min(n_lon);
            max_lon = max_lon.max(n_lon);
        }

        Some(Self::new(min_lat, min_lon, max_lat, max_lon))
    }

    /// Returns true if this envelope contains the given point.
    pub fn contains_point(&self, lat: f64, lon: f64) -> bool {
        let n_lon = normalize_lon(lon);
        lat >= self.min_lat && lat <= self.max_lat && n_lon >= self.min_lon && n_lon <= self.max_lon
    }

    /// Returns true if this envelope fully encloses another envelope.
    pub fn contains_envelope(&self, other: &Self) -> bool {
        self.min_lat <= other.min_lat
            && self.max_lat >= other.max_lat
            && self.min_lon <= other.min_lon
            && self.max_lon >= other.max_lon
    }

    /// Returns true if this envelope intersects with another envelope.
    pub fn intersects(&self, other: &Self) -> bool {
        self.min_lat <= other.max_lat
            && self.max_lat >= other.min_lat
            && self.min_lon <= other.max_lon
            && self.max_lon >= other.min_lon
    }

    /// Computes the union envelope of this and another envelope.
    pub fn merge(&self, other: &Self) -> Self {
        Self::new(
            self.min_lat.min(other.min_lat),
            self.min_lon.min(other.min_lon),
            self.max_lat.max(other.max_lat),
            self.max_lon.max(other.max_lon),
        )
    }

    /// Computes the area metric of the bounding box in degrees squared.
    pub fn area(&self) -> f64 {
        let d_lat = (self.max_lat - self.min_lat).max(0.0);
        let d_lon = (self.max_lon - self.min_lon).max(0.0);
        d_lat * d_lon
    }

    /// Computes the semi-perimeter margin of the bounding box.
    pub fn margin(&self) -> f64 {
        let d_lat = (self.max_lat - self.min_lat).max(0.0);
        let d_lon = (self.max_lon - self.min_lon).max(0.0);
        d_lat + d_lon
    }

    /// Computes center coordinate [lat, lon] of the envelope.
    pub fn center(&self) -> [f64; 2] {
        [
            (self.min_lat + self.max_lat) * 0.5,
            (self.min_lon + self.max_lon) * 0.5,
        ]
    }

    /// Computes minimal geodesic distance in meters from point to the envelope boundary.
    pub fn distance_to_point(&self, lat: f64, lon: f64) -> f64 {
        if self.contains_point(lat, lon) {
            return 0.0;
        }
        let clamped_lat = lat.clamp(self.min_lat, self.max_lat);
        let clamped_lon = normalize_lon(lon).clamp(self.min_lon, self.max_lon);
        haversine_distance_m(lat, lon, clamped_lat, clamped_lon)
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

    fn search_node<'a>(
        node: &'a RTreeNode<T>,
        query: &SpatialEnvelope,
        results: &mut Vec<&'a T>,
    ) {
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
