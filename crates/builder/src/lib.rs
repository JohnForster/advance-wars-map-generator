use rand::{self, Rng};
use visualiser::Visualiser;
use world_core::{join_tile_collections, Coordinates, Player, Players, Symmetry, TileType, World};
mod tile_path;
pub mod utils;
use tile_path::*;

pub struct WorldBuilder {
    pub world: World,
    pub players: Players,
    pub symmetry: Symmetry,
    pub visualiser: Option<Visualiser>,
}

pub struct Config {
    pub height: u32,
    pub width: u32,
}

#[allow(unused_mut)]
impl WorldBuilder {
    pub fn new(config: Config) -> WorldBuilder {
        let tiles: Vec<TileType> = vec![TileType::Empty; (config.height * config.width) as usize];
        WorldBuilder {
            world: World {
                height: config.height,
                width: config.width,
                tiles,
            },
            players: Players::Two,
            symmetry: Symmetry::Rotational,
            visualiser: None,
        }
    }

    pub fn get_reciprocal(&self, coords: &Coordinates) -> Coordinates {
        match self.symmetry {
            Symmetry::Rotational => {
                let x = self.world.width - coords.x - 1;
                let y = self.world.height - coords.y - 1;
                Coordinates::new(x, y)
            }
            _ => todo!(),
        }
    }
    pub fn get_reciprocal_by_index(&self, i: usize) -> usize {
        let coordinates = self.world.coords_from_index(i);
        let rcp_coords = self.get_reciprocal(&coordinates);
        return self.world.index_from_coords(&rcp_coords);
    }

    pub fn with_visualiser(mut self, visualiser: Visualiser) -> WorldBuilder {
        self.visualiser = Some(visualiser);
        self
    }

    pub fn fill(mut self, tile_type: TileType) -> WorldBuilder {
        self.scatter(tile_type, 1.0);
        self
    }
    pub fn set_players(mut self, players: Players) -> WorldBuilder {
        self.players = players;
        self
    }
    pub fn set_symmetry(mut self, symmetry: Symmetry) -> WorldBuilder {
        self.symmetry = symmetry;
        self
    }
    pub fn create_headquarters(mut self) -> WorldBuilder {
        // TODO Add player number blocks.
        let (x, y, r_x, r_y) = loop {
            let x = rand::thread_rng().gen_range(0..self.world.width);
            let y = rand::thread_rng().gen_range(0..self.world.height);
            let Coordinates { x: rx, y: ry } = match self.symmetry {
                Symmetry::Rotational => self.get_reciprocal(&Coordinates::new(x, y)),
                _ => todo!(),
            };

            if rx != x && ry != y {
                break (x, y, rx, ry);
            }
        };

        let id = self.world.index_from_coords(&Coordinates::new(x, y));
        let r_id = self.world.index_from_coords(&Coordinates::new(r_x, r_y));
        self.world.tiles[id] = TileType::Hq(Player::One);
        self.world.tiles[r_id] = TileType::Hq(Player::Two);

        self
    }
    pub fn join_headquarters(mut self, tile_type: TileType) -> WorldBuilder {
        let hq_locations = self.world.find_headquarters();

        if hq_locations.len() == 0 {
            println!("WARNING: Couldn't find headquarters. Continuing...");
            return self;
        }

        let hq_1 = hq_locations[0];
        let hq_2 = hq_locations[1];
        let path = TilePath::generate(hq_1, hq_2, self.symmetry, &self, tile_type);

        match path {
            Ok(path) => {
                let all_tiles: Vec<_> =
                    path.tiles.iter().zip(path.reciprocal_path.iter()).collect();
                for (tile, reciprocal) in all_tiles {
                    self.world.update_tile(&tile, tile_type.clone());
                    self.world.update_tile(&reciprocal, tile_type.clone());
                }
            }
            Err(err) => match err {
                PathGenerationError::InfiniteLoopErr(tiles, reciprocals)
                | PathGenerationError::NoNextTileErr(tiles, reciprocals) => {
                    println!("Error generating path");
                    let all_tiles: Vec<_> = tiles.iter().zip(reciprocals.iter()).collect();
                    for (tile, reciprocal) in all_tiles {
                        self.world.update_tile(&tile, TileType::Road);
                        self.world.update_tile(&reciprocal, TileType::Road);
                    }
                }
            },
        }
        self
    }

    pub fn create_team_cities(mut self) -> WorldBuilder {
        println!("Not yet implemented");
        self
    }
    pub fn create_neutral_cities(mut self) -> WorldBuilder {
        println!("Not yet implemented");
        self
    }
    pub fn create_team_factories(mut self) -> WorldBuilder {
        println!("Not yet implemented");
        self
    }
    pub fn create_neutral_factories(mut self) -> WorldBuilder {
        println!("Not yet implemented");
        self
    }
    pub fn create_roads(mut self) -> WorldBuilder {
        println!("Not yet implemented");
        self
    }
    pub fn add_seas(mut self) -> WorldBuilder {
        println!("Not yet implemented");
        self
    }
    pub fn add_forests(mut self, density: f32) -> WorldBuilder {
        self.scatter(TileType::Forest, density);
        self
    }
    pub fn add_mountains(mut self, density: f32) -> WorldBuilder {
        self.scatter(TileType::Mountain, density);
        self
    }
    pub fn build(self) -> World {
        self.world
    }

    fn scatter(&mut self, scatter_type: TileType, density: f32) {
        if density > 1.0 {
            println!("Density must be between 0.0 and 1.0. Skipping...");
            return;
        }

        for i in 0..self.world.tiles.len() {
            let rcp_i = self.get_reciprocal_by_index(i);
            let tile_type = self.world.tile_at(i);
            let rcp = self.world.tile_at(rcp_i);
            if tile_type.is_empty() && rcp.is_empty() {
                let x = rand::thread_rng().gen_range(0.0..1.0);
                if x < density {
                    self.world.update_tile_by_index(i, scatter_type);
                    self.world.update_tile_by_index(rcp_i, scatter_type);
                }
            }
        }
    }
}
