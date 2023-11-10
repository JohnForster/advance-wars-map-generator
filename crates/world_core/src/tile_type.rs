#[derive(Debug, Copy, Clone)]
pub enum Player {
    One,
    Two,
    _Three,
    _Four,
}

#[derive(Debug, Clone)]
pub enum TileType {
    Plains,
    Sea,
    Mountain,
    Forest,
    Road,
    City(Option<Player>),
    Factory(Option<Player>),
    _Port(Option<Player>),
    _Airport(Option<Player>),
    Hq(Player),
    _Reef,
    Empty,
}

pub type TileTypeId = usize;

impl TileType {
    pub fn from_id(id: TileTypeId, allegiance: Option<Player>) -> TileType {
        if let Some(player) = allegiance {
            return match id {
                6 => TileType::City(Some(player)),
                7 => TileType::Factory(Some(player)),
                8 => TileType::Hq(player),
                _ => panic!("Unknown id: {:?}", id),
            };
        }

        return match id {
            1 => TileType::Plains,
            2 => TileType::Sea,
            3 => TileType::Forest,
            4 => TileType::Mountain,
            5 => TileType::Road,
            6 => TileType::City(allegiance),
            7 => TileType::Factory(allegiance),
            _ => panic!("Unknown id: {:?}", id),
        };
    }

    pub fn to_id(&self) -> TileTypeId {
        match self {
            TileType::Plains => 1,
            TileType::Sea => 2,
            TileType::Forest => 3,
            TileType::Mountain => 4,
            TileType::Road => 5,
            TileType::City(_) => 6,
            TileType::Factory(_) => 7,
            TileType::Hq(_) => 8,
            _ => panic!("No id for tile {:?}", self),
        }
    }
}
