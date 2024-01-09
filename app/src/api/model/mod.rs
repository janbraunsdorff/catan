pub mod game;

use catan_core::game::TileType;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub x: u8,
    pub y: u8,
    pub tile_type: TileType,
    pub dice_value: u8 
}