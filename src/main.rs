use engine::{Config, WorldBuilder};
use visualiser::Visualiser;
use world_core::{Players, Symmetry, TileType};

fn main() {
    let config = Config {
        height: 21,
        width: 21,
    };
    let world_builder = WorldBuilder::new(config);
    let visualiser = Visualiser::new();
    println!("Running!");
    let world = world_builder
        .with_visualiser(visualiser)
        .set_players(Players::Two)
        .set_symmetry(Symmetry::Rotational)
        .create_headquarters()
        .join_headquarters(TileType::Road)
        .add_forests(0.1)
        .add_mountains(0.05)
        .fill(TileType::Plains)
        // .create_team_cities()
        // .create_neutral_cities()
        // .create_team_factories()
        // .create_neutral_factories()
        // .create_roads()
        // .add_seas()
        .build();

    let visualiser = Visualiser::new();
    visualiser.visualise(&world)
}
