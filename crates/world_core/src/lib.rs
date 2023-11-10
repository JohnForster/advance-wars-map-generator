pub mod parser;
mod tile_type;
pub use tile_type::*;

pub struct World {
    pub tiles: Vec<TileType>,
    pub width: u32,
    pub height: u32,
}

impl World {
    pub fn index_from_coords(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }
    pub fn coords_from_index(&self, i: usize) -> (u32, u32) {
        let y = i as u32 / self.width;
        let x = i as u32 % self.width;
        (x, y)
    }
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct TileData {
    pub id: TileTypeId,
    pub name: String,
    pub colour: String,
}

pub enum Players {
    Two,
    _Three,
    _Four,
}

pub enum Symmetry {
    Rotational,
    Horizontal,
    Vertical,
}
