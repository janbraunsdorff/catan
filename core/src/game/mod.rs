use serde::{Deserialize, Serialize};

use self::board::Board;

pub mod board;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Color {
    RED,
    BLUE,
    ORANGE,
    WHITHE,
    DEFAULT,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TileType {
    Hills,
    Forest,
    Mountains,
    Fields,
    Pasture,
    Dessert,
    Water,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PortType {
    ANY,
    WOOL,
    LUMBER,
    GRAIN,
    ORE,
    BRICKS,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub color: Color,
    pub name: String,
    pub npc: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BuildingType {
    EMPTY,
    SETTELMENT,
    TOWN,
}

pub struct Game {
    pub board: Option<Board>,
    pub players: Vec<Player>,
    pub extenstions: Vec<String>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: None,
            players: Vec::new(),
            extenstions: Vec::new(),
        }
    }
}
