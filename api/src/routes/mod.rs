pub mod create;
pub mod state;



use game::game::TileType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub x: u8,
    pub y: u8,
    pub tile_type: TileType,
    pub dice_value: u8,
}
