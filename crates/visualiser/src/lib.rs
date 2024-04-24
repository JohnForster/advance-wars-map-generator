use core::time;
use std::thread::sleep;

use colored::{ColoredString, Colorize};
use world_core::{parser::Parser, Player, TileCollection, TileType, World};

pub enum VisualiserType {
    Terminal,
    PNG,
}

pub struct Visualiser {
    parser: Parser,
    visualiser_type: VisualiserType,
}

impl Visualiser {
    pub fn new() -> Visualiser {
        let parser = Parser::new();
        Visualiser {
            parser,
            visualiser_type: VisualiserType::Terminal,
        }
    }

    pub fn visualise_with_extra_tiles(&self, world: &World, tile_collection: &TileCollection) {
        match self.visualiser_type {
            VisualiserType::Terminal => self.print_to_terminal(world, Some(tile_collection)),
            _ => todo!(),
        }
    }

    pub fn visualise(&self, world: &World) {
        match self.visualiser_type {
            VisualiserType::Terminal => self.print_to_terminal(world, None),
            _ => todo!(),
        }
    }

    pub fn print_to_terminal(&self, world: &World, tile_collection: Option<&TileCollection>) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let new_world = tile_collection.map(|tc| {
            let mut world = world.clone();
            world.apply_tile_collection(&tc);
            world
        });

        let world = new_world.as_ref().unwrap_or(world);

        let bar = String::from('═').repeat(world.width as usize * 2 + 1);
        let top_header = format!("{}{}{}", '╔', bar, '╗');
        let bottom_header = format!("{}{}{}", '╚', bar, '╝');
        println!("{}", top_header);
        for chunk in world.tiles.chunks(world.width as usize) {
            print!("║ ");
            for tile_type in chunk {
                let str = terminal_symbol(tile_type);
                print!("{str}");
            }
            print!("║\n");
        }
        println!("{}", bottom_header);
        let ten_millis = time::Duration::from_millis(10);
        sleep(ten_millis)
    }
}

fn colour_player_string(string: &str, player: &Option<Player>) -> ColoredString {
    if let Some(player) = player {
        return match player {
            Player::One => string.truecolor(255, 200, 0),
            Player::Two => string.truecolor(0, 200, 255),
            _ => todo!(),
        };
    } else {
        return string.truecolor(200, 200, 200);
    }
}

fn terminal_symbol(tile_type: &TileType) -> ColoredString {
    match tile_type {
        TileType::Plains => "▓▓".green().bold(),
        TileType::Mountain => "▓▓".yellow(),
        TileType::Sea => "▓▓".blue(),
        TileType::City(player) => colour_player_string("▓▓", player),
        TileType::Road => "▓▓".truecolor(3, 3, 3),
        TileType::Forest => "▓▓".truecolor(0, 90, 0),
        TileType::Factory(player) => colour_player_string("★ ", player),
        TileType::Hq(player) => colour_player_string("★ ", &Some(*player)),
        TileType::Empty => "□ ".white(),
        _ => "? ".bold().bright_magenta(),
    }
}
