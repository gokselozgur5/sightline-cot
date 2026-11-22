//! Geohash neighbor cell calculation for spatial search.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeohashDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeohashNeighbors {
    pub north: String,
    pub north_east: String,
    pub east: String,
    pub south_east: String,
    pub south: String,
    pub south_west: String,
    pub west: String,
    pub north_west: String,
}

pub use crate::geohash::GeohashError;

const BASE32: &[u8; 32] = b"0123456789bcdefghjkmnpqrstuvwxyz";

const BORDERS: [(&[u8], &[u8]); 4] = [
    (b"prxz", b"bcfguvyz"),
    (b"028b", b"0145hjnp"),
    (b"bcfguvyz", b"prxz"),
    (b"0145hjnp", b"028b"),
];

const NEIGHBORS: [(&[u8; 32], &[u8; 32]); 4] = [
    (
        b"p0r21436x8zb9dcf5h7kjnmqesgutwvy",
        b"bc01fg45238967deuvhjyznpkmstqrwx",
    ),
    (
        b"14365h7k9dcfesgujnmqp0r2twvyx8zb",
        b"238967debc01fg45kmstqrwxuvhjyznp",
    ),
    (
        b"bc01fg45238967deuvhjyznpkmstqrwx",
        b"p0r21436x8zb9dcf5h7kjnmqesgutwvy",
    ),
    (
        b"238967debc01fg45kmstqrwxuvhjyznp",
        b"14365h7k9dcfesgujnmqp0r2twvyx8zb",
    ),
];

fn validate_geohash(hash: &str) -> Result<(), GeohashError> {
    if hash.is_empty() {
        return Err(GeohashError::EmptyHash);
    }
    for b in hash.bytes() {
        let lower = b.to_ascii_lowercase();
        if !BASE32.contains(&lower) {
            return Err(GeohashError::InvalidCharacter(b as char));
        }
    }
    Ok(())
}

fn neighbor_cardinal(hash: &str, dir_idx: usize) -> Result<String, GeohashError> {
    let bytes = hash.as_bytes();
    let last = bytes[bytes.len() - 1].to_ascii_lowercase();
    let is_odd = (bytes.len() % 2) == 1;
    let (border, table) = if is_odd {
        (BORDERS[dir_idx].1, NEIGHBORS[dir_idx].1)
    } else {
        (BORDERS[dir_idx].0, NEIGHBORS[dir_idx].0)
    };
    let parent = &hash[..hash.len() - 1];
    let base_parent = if border.contains(&last) {
        if parent.is_empty() {
            if dir_idx <= 1 {
                return Err(GeohashError::AtPole);
            }
            String::new()
        } else {
            neighbor_cardinal(parent, dir_idx)?
        }
    } else {
        parent.to_ascii_lowercase()
    };
    let idx = table
        .iter()
        .position(|&b| b == last)
        .ok_or(GeohashError::InvalidCharacter(last as char))?;
    let mut res = base_parent;
    res.push(BASE32[idx] as char);
    Ok(res)
}

/// Calculate neighbor geohash cell in a specific direction.
pub fn geohash_neighbor(hash: &str, dir: GeohashDirection) -> Result<String, GeohashError> {
    validate_geohash(hash)?;
    match dir {
        GeohashDirection::North => neighbor_cardinal(hash, 0),
        GeohashDirection::South => neighbor_cardinal(hash, 1),
        GeohashDirection::East => neighbor_cardinal(hash, 2),
        GeohashDirection::West => neighbor_cardinal(hash, 3),
        GeohashDirection::NorthEast => {
            let n = neighbor_cardinal(hash, 0)?;
            neighbor_cardinal(&n, 2)
        }
        GeohashDirection::NorthWest => {
            let n = neighbor_cardinal(hash, 0)?;
            neighbor_cardinal(&n, 3)
        }
        GeohashDirection::SouthEast => {
            let s = neighbor_cardinal(hash, 1)?;
            neighbor_cardinal(&s, 2)
        }
        GeohashDirection::SouthWest => {
            let s = neighbor_cardinal(hash, 1)?;
            neighbor_cardinal(&s, 3)
        }
    }
}

/// Calculate all 8 surrounding neighbor cells for spatial search.
pub fn geohash_neighbors(hash: &str) -> Result<GeohashNeighbors, GeohashError> {
    validate_geohash(hash)?;
    let north = neighbor_cardinal(hash, 0)?;
    let south = neighbor_cardinal(hash, 1)?;
    let east = neighbor_cardinal(hash, 2)?;
    let west = neighbor_cardinal(hash, 3)?;
    let north_east = neighbor_cardinal(&north, 2)?;
    let north_west = neighbor_cardinal(&north, 3)?;
    let south_east = neighbor_cardinal(&south, 2)?;
    let south_west = neighbor_cardinal(&south, 3)?;
    Ok(GeohashNeighbors {
        north,
        north_east,
        east,
        south_east,
        south,
        south_west,
        west,
        north_west,
    })
}
