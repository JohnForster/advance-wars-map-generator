use std::collections::HashMap;

use crate::TileData;

type Tiles = HashMap<usize, TileData>;

pub struct Parser {
    pub data: Tiles,
}

impl Parser {
    const TILES_CSV: &str = include_str!("../data/tiles.csv");

    pub fn new() -> Parser {
        Parser {
            data: Parser::load_data(),
        }
    }

    pub fn load_data() -> Tiles {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(Self::TILES_CSV.as_bytes());
        let mut all_tile_data = HashMap::new();
        for result in reader.deserialize() {
            let tile_data: TileData = result.unwrap();
            all_tile_data.insert(tile_data.id, tile_data);
        }
        return all_tile_data;
    }
}
