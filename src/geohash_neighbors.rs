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
