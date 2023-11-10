use rand::{self, Rng};
use world_core::{Player, Players, Symmetry, TileType, TileTypeId, World};

pub struct WorldBuilder {
    pub world: World,
    pub players: Players,
    pub symmetry: Symmetry,
}

pub struct Config {
    pub height: u32,
    pub width: u32,
}

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
        }
    }

    pub fn fill(mut self, tile_type: TileType) -> WorldBuilder {
        self.world.tiles.fill(tile_type);
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
        let x = rand::thread_rng().gen_range(0..self.world.width);
        let y = rand::thread_rng().gen_range(0..self.world.height);

        // TODO Add player number blocks.
        let (r_x, r_y) = loop {
            let (rx, ry) = match self.symmetry {
                Symmetry::Rotational => (self.world.width - x - 1, self.world.height - y - 1),
                _ => todo!(),
            };

            if rx != x && ry != y {
                break (rx, ry);
            }
        };

        let id = self.world.index_from_coords(x, y);
        let r_id = self.world.index_from_coords(r_x, r_y);
        self.world.tiles[id] = TileType::Hq(Player::One);
        self.world.tiles[r_id] = TileType::Hq(Player::Two);

        self
    }
    pub fn join_headquarters(mut self) -> WorldBuilder {
        todo!("Not yet implemented")
    }
    pub fn create_team_cities(mut self) -> WorldBuilder {
        todo!("Not yet implemented")
    }
    pub fn create_neutral_cities(mut self) -> WorldBuilder {
        todo!("Not yet implemented")
    }
    pub fn create_team_factories(mut self) -> WorldBuilder {
        todo!("Not yet implemented")
    }
    pub fn create_neutral_factories(mut self) -> WorldBuilder {
        todo!("Not yet implemented")
    }
    pub fn create_roads(mut self) -> WorldBuilder {
        todo!("Not yet implemented")
    }
    pub fn add_seas(mut self) -> WorldBuilder {
        todo!("Not yet implemented")
    }
    pub fn add_forests(mut self) -> WorldBuilder {
        todo!("Not yet implemented")
    }
    pub fn add_mountains(mut self) -> WorldBuilder {
        todo!("Not yet implemented")
    }
    pub fn build(self) -> World {
        self.world
    }
}
