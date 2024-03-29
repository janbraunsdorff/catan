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
    HILLS,
    FOREST,
    MOUNTAINS,
    FIELD,
    PASTURE,
    DESSERT,
    WATER,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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

#[derive(Default)]
pub struct Game {
    pub board: Option<Board>,
    pub players: Vec<Player>,
    pub extenstions: Vec<String>,
}
