use std::f32::consts::PI;

use world_core::{Coordinates, Symmetry, TileCollection, TileType};

use crate::{utils, WorldBuilder};

pub struct TilePath<'a> {
    pub tiles: Vec<Coordinates>,
    pub reciprocal_path: Vec<Coordinates>,
    from: Coordinates,
    to: Coordinates,
    builder: &'a WorldBuilder,
    symmetry: Symmetry,
    tile_type: TileType,
}

pub struct NextStepErr;
pub enum PathGenerationError {
    NoNextTileErr(Vec<Coordinates>, Vec<Coordinates>),
    InfiniteLoopErr(Vec<Coordinates>, Vec<Coordinates>),
}

impl TilePath<'_> {
    fn next_step(&mut self) -> Result<(), PathGenerationError> {
        let last_tile = self.tiles.last().unwrap_or(&self.from);
        let neighbours = self
            .builder
            .world
            .get_neighbours(last_tile)
            .iter()
            .filter_map(|&x| x)
            .collect::<Vec<_>>();

        let weights = neighbours
            .iter()
            .map(|n| self.calculate_weight(&n))
            .collect::<Vec<f32>>();

        let tile = utils::choose(&neighbours, &weights);
        if tile.is_none() {
            if self.tiles.iter().any(|tile_1| {
                self.reciprocal_path
                    .iter()
                    .any(|tile_2| tile_1.neighbours(tile_2))
            }) {
                return Ok(());
            }

            return Err(PathGenerationError::NoNextTileErr(
                self.tiles.clone(),
                self.reciprocal_path.clone(),
            ));
        }
        let tile = tile.unwrap();
        self.tiles.push(*tile);
        self.reciprocal_path.push(self.builder.get_reciprocal(tile));

        if let Some(visualiser) = &self.builder.visualiser {
            visualiser.visualise_with_extra_tiles(&self.builder.world, &self.to_tile_collection());
        }
        return Ok(());
    }

    pub fn is_complete(&mut self) -> bool {
        let last_tile = self.tiles.last().unwrap_or(&self.from);
        let path_meets_destination = last_tile.neighbours(&self.to);
        let paths_overlap = self
            .tiles
            .iter()
            .any(|t| self.reciprocal_path.iter().any(|t2| t.neighbours(t2)));
        return path_meets_destination || paths_overlap;
    }

    pub fn delete_last(&mut self, n: u32) {
        for _ in 0..n {
            self.tiles.pop();
            self.reciprocal_path.pop();
        }
        println!("self.tiles.last(): {:?}", self.tiles.last());
    }

    fn calculate_weight(&self, tile: &Coordinates) -> f32 {
        if tile == &self.to {
            return f32::MAX;
        }

        // If tile is already in path, weight is 0
        let tile_in_world = self.builder.world.tile_at_coords(tile);
        if !tile_in_world.is_empty() || self.tiles.contains(tile) || tile == &self.from {
            return 0.0;
        };

        // Remove if tile has two road neighbours.
        let world = &self.builder.world;
        let neighbours = world.get_neighbours(tile);

        if neighbours
            .iter()
            .filter_map(|&n| n)
            .filter(|n| {
                self.tiles.contains(&n)
                    // || self.reciprocal_path.contains(&n)
                    || n == &self.from
                    || n == &self.to
            })
            .count()
            > 1
        {
            return 0.0;
        };

        // Compare angle.
        let previous = self.tiles.last().unwrap_or(&self.from);
        let end = self.to;
        let diff = calculate_difference_in_angles(tile, previous, &end);

        let weight = 0.4 * diff.cos() + 0.6;

        return weight;
    }

    pub fn generate(
        from: Coordinates,
        to: Coordinates,
        symmetry: Symmetry,
        builder: &WorldBuilder,
        tile_type: TileType,
    ) -> Result<TilePath, PathGenerationError> {
        let mut path = TilePath {
            from,
            to,
            builder,
            symmetry,
            tile_type,
            tiles: Vec::new(),
            reciprocal_path: Vec::new(),
        };

        println!(
            "Generating path from (path.from, path.to): {:?}",
            (path.from, path.to)
        );

        let mut err_count = 0;
        let count = 0;
        while !path.is_complete() {
            if count > 10_000 {
                println!("Infinite loop!");
                return Err(PathGenerationError::InfiniteLoopErr(
                    path.tiles,
                    path.reciprocal_path,
                ));
            }
            let result = path.next_step();
            if let Err(err) = result {
                if err_count > 100 {
                    return Err(err);
                }
                println!("err_count: {:?}", err_count);
                path.delete_last(err_count);
                err_count += 1;
            }
        }

        Ok(path)
    }

    pub fn to_tile_collection(&self) -> TileCollection {
        let mut tile_collection = TileCollection::new();
        for tile in &self.tiles {
            let index = self.builder.world.index_from_coords(&tile);
            tile_collection.push((index, self.tile_type));
        }
        for tile in &self.reciprocal_path {
            let index = self.builder.world.index_from_coords(&tile);
            tile_collection.push((index, self.tile_type));
        }
        return tile_collection;
    }
}

fn calculate_difference_in_angles(
    tile: &Coordinates,
    previous: &Coordinates,
    end: &Coordinates,
) -> f32 {
    let vector_from_previous = previous.vector_to(tile);
    let vector_to_end = tile.vector_to(end);
    let diff = vector_to_end.diff(&vector_from_previous);

    if diff.angle <= PI {
        return diff.angle;
    } else {
        return (2.0 * PI) - diff.angle;
    }
}

#[test]
fn test_angle_difference_same() {
    let tile = Coordinates::new(2, 10);
    let previous = Coordinates::new(1, 10);
    let end = Coordinates::new(3, 10);
    let result = calculate_difference_in_angles(&tile, &previous, &end);
    assert_eq!(result, 0.0)
}

#[test]
fn test_angle_difference_opposite() {
    let tile = Coordinates::new(2, 10);
    let previous = Coordinates::new(4, 10);
    let end = Coordinates::new(3, 10);
    let result = calculate_difference_in_angles(&tile, &previous, &end);
    assert_eq!(result, PI)
}

#[test]
fn test_angle_difference_orthoganal() {
    let tile_1 = Coordinates::new(2, 4);
    let tile_2 = Coordinates::new(2, 6);
    let previous = Coordinates::new(2, 5);
    let end = Coordinates::new(8, 5);
    let result_1 = calculate_difference_in_angles(&tile_1, &previous, &end);
    let result_2 = calculate_difference_in_angles(&tile_2, &previous, &end);
    assert_eq!(result_1, PI / 2.0);
    assert_eq!(result_2, PI / 2.0);
    assert_eq!(result_1, result_2);
}

#[test]
fn test_angle_difference_acute() {
    let tile_1 = Coordinates::new(3, 4);
    let tile_2 = Coordinates::new(4, 6);
    let previous = Coordinates::new(2, 5);
    let end = Coordinates::new(8, 5);
    let result_1 = calculate_difference_in_angles(&tile_1, &previous, &end);
    let result_2 = calculate_difference_in_angles(&tile_2, &previous, &end);
    assert_eq!(result_1, result_2);
}
