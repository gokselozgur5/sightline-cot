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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeohashError {
    EmptyString,
    InvalidCharacter(char),
}

impl std::fmt::Display for GeohashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyString => write!(f, "geohash string cannot be empty"),
            Self::InvalidCharacter(c) => write!(f, "invalid geohash character '{c}'"),
        }
    }
}

impl std::error::Error for GeohashError {}

