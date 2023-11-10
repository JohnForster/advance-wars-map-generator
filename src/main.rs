use engine::{Config, WorldBuilder};
use visualiser::Visualiser;
use world_core::{Players, Symmetry, TileType};

fn main() {
    let config = Config {
        height: 20,
        width: 20,
    };
    let world_builder = WorldBuilder::new(config);
    let visualiser = Visualiser::new();

    let world = world_builder
        .fill(TileType::Plains)
        .set_players(Players::Two)
        .set_symmetry(Symmetry::Rotational)
        .create_headquarters()
        // .join_headquarters()
        // .create_team_cities()
        // .create_neutral_cities()
        // .create_team_factories()
        // .create_neutral_factories()
        // .create_roads()
        // .add_seas()
        // .add_forests()
        // .add_mountains()
        .build();

    visualiser.print(world);
}
