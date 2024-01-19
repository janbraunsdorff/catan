use axum::http::StatusCode;
use game::{eventque::start::TileEvent, game::TileType};
use serde::{Deserialize, Serialize};

use crate::error::ExternalExecutionError;

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tile_type: String,
    pub dice_value: u8,
}

impl Tile {
    pub fn to_event(&self) -> Result<TileEvent, ExternalExecutionError> {
        let ttype = match tile_type_from_string(&self.tile_type) {
            Ok(val) => val,
            Err(err) => return Err(err),
        };

        Ok(TileEvent {
            idx: self.y * 100 + self.x,
            x: self.x,
            y: self.y,
            tile_type: ttype,
            dice: self.dice_value,
        })
    }
}

fn tile_type_from_string(tile_type: &str) -> Result<TileType, ExternalExecutionError> {
    match tile_type.to_uppercase().as_str() {
        "HILLS" => Ok(TileType::HILLS),
        "FOREST" => Ok(TileType::FOREST),
        "MOUNTAINS" => Ok(TileType::MOUNTAINS),
        "FIELD" => Ok(TileType::FIELD),
        "PASTURE" => Ok(TileType::PASTURE),
        "DESSERT" => Ok(TileType::DESSERT),
        "WATER" => Ok(TileType::WATER),
        _ => Err(ExternalExecutionError {
            status_code: StatusCode::BAD_GATEWAY,
            message: "Color is missing or not found".to_string(),
            step: "parse player color".to_string(),
        }),
    }
}
