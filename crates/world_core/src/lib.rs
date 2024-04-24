pub mod parser;
mod tile_type;
use std::f32::consts::PI;

pub use tile_type::*;

#[derive(Clone)]
pub struct World {
    pub tiles: Vec<TileType>,
    pub width: u32,
    pub height: u32,
}

pub type Neighbours = [Option<Coordinates>; 4];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct Vector {
    pub length: f32,
    pub angle: f32,
}

pub type TileCollection = Vec<(usize, TileType)>;
pub fn join_tile_collections(tc_a: TileCollection, mut tc_b: TileCollection) -> TileCollection {
    let mut output = tc_a.clone();
    output.append(&mut tc_b);
    return output;
}

impl Vector {
    pub fn diff(&self, other: &Vector) -> Vector {
        Vector {
            length: other.length - self.length,
            angle: (other.angle - self.angle).rem_euclid(2.0 * PI),
        }
    }
}

impl Coordinates {
    pub fn new(x: u32, y: u32) -> Coordinates {
        Coordinates { x, y }
    }

    pub fn distance(&self, other: &Coordinates) -> f32 {
        let dx = self.x as i32 - other.x as i32;
        let dy = self.y as i32 - other.y as i32;
        let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();
        return distance;
    }

    pub fn vector_to(&self, other: &Coordinates) -> Vector {
        let dx = (other.x as i32 - self.x as i32) as f32;
        let dy = (other.y as i32 - self.y as i32) as f32;
        let length = self.distance(other);
        let angle = dx.atan2(dy);
        return Vector { length, angle };
    }

    pub fn neighbours(&self, other: &Coordinates) -> bool {
        return self.distance(other) == 1.0;
    }
}

impl World {
    pub fn index_from_coords(&self, coords: &Coordinates) -> usize {
        if coords.x >= self.width || coords.y >= self.height {
            panic!(
                "Trying to read coords ({}, {}) for world of size ({},{})",
                coords.x, coords.y, self.width, self.height
            )
        }
        (coords.y * self.width + coords.x) as usize
    }

    pub fn coords_from_index(&self, i: usize) -> Coordinates {
        let y = i as u32 / self.width;
        let x = i as u32 % self.width;
        Coordinates::new(x, y)
    }

    // ? Should this return an index instead of coords?
    // ? Should we saving HQ coords in state?
    pub fn find_headquarters(&self) -> Vec<Coordinates> {
        let mut coordinates: Vec<Coordinates> = Vec::new();
        for (i, tile) in self.tiles.iter().enumerate() {
            if let TileType::Hq(_) = tile {
                coordinates.push(self.coords_from_index(i))
            }
        }
        return coordinates;
    }

    pub fn get_neighbours(&self, &Coordinates { x, y }: &Coordinates) -> Neighbours {
        let right = (x + 1 < self.width).then(|| Coordinates::new(x + 1, y));
        let left = (x as i32 - 1 >= 0).then(|| Coordinates::new(x - 1, y));
        let up = (y as i32 - 1 >= 0).then(|| Coordinates::new(x, y - 1));
        let down = (y + 1 < self.height).then(|| Coordinates::new(x, y + 1));

        return [up, down, left, right];
    }

    pub fn update_tile(&mut self, coords: &Coordinates, tile_type: TileType) {
        let i = self.index_from_coords(coords);
        self.tiles[i] = tile_type;
    }

    pub fn update_tile_by_index(&mut self, index: usize, tile_type: TileType) {
        self.tiles[index] = tile_type;
    }

    pub fn apply_tile_collection(&mut self, tile_collection: &TileCollection) {
        for (index, tile_type) in tile_collection {
            self.update_tile_by_index(*index, *tile_type)
        }
    }

    pub fn tile_at(&self, i: usize) -> &TileType {
        return &self.tiles[i];
    }
    pub fn tile_at_coords(&self, coords: &Coordinates) -> &TileType {
        let i = self.index_from_coords(coords);
        return &self.tiles[i];
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

#[derive(Copy, Clone)]
pub enum Symmetry {
    Rotational,
    Horizontal,
    Vertical,
}

#[test]
fn test_vector() {
    let a = Coordinates::new(2, 2);
    let b = Coordinates::new(4, 4);
    let v = a.vector_to(&b);
    assert_eq!(v.angle, PI / 4.0);
}

#[test]
fn test_coordinates() {
    let c_1: Coordinates = Coordinates::new(9, 11);
    let c_2: Coordinates = Coordinates::new(10, 11);
    let c_3: Coordinates = Coordinates::new(10, 12);
    let c_4: Coordinates = Coordinates::new(9, 12);

    assert!(c_1.neighbours(&c_2));
    assert!(c_1.neighbours(&c_4));
    assert!(c_2.neighbours(&c_1));
    assert!(c_2.neighbours(&c_3));
    assert!(c_3.neighbours(&c_2));
    assert!(c_3.neighbours(&c_4));
    assert!(c_4.neighbours(&c_1));
    assert!(c_4.neighbours(&c_3));
}
