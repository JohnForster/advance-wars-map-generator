use colored::{ColoredString, Colorize};
use world_core::{parser::Parser, Player, TileType, World};

pub struct Visualiser {
    parser: Parser,
}

impl Visualiser {
    pub fn new() -> Visualiser {
        let parser = Parser::new();
        Visualiser { parser }
    }

    pub fn print(&self, world: World) {
        let tile_data = &self.parser.data;

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("tiles:");
        for chunk in world.tiles.chunks(world.width as usize) {
            for tile_type in chunk {
                let str = match tile_type {
                    TileType::Plains => "■ ".green().bold(),
                    TileType::Mountain => "■ ".yellow(),
                    TileType::Sea => "■ ".blue(),
                    TileType::City(player) => colour_player_string("■ ", player),
                    TileType::Road => "■ ".truecolor(3, 3, 3),
                    TileType::Forest => "■ ".truecolor(0, 90, 0),
                    TileType::Factory(player) => colour_player_string("★ ", player),
                    TileType::Hq(player) => colour_player_string("★ ", &Some(*player)),
                    // "_empty" => "□ ".white(),
                    _ => "? ".bold().bright_magenta(),
                };

                print!("{str}");
            }
            print!("\n");
        }
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
