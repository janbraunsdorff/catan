pub mod create;
pub mod state;
use axum::http::StatusCode;

use game::{game::{TileType, PortType, BuildingType}, eventque::start::{BuildingEvent, TileEvent, PortEvent}};
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
    fn to_event(&self) -> Result<TileEvent, ExternalExecutionError> {
        let ttype = match tile_type_from_string(&self.tile_type){
            Ok(val) => val,
            Err(err) => return Err(err),
        };

        Ok(TileEvent{
            idx: self.y * 100 + self.x,
            x: self.x,
            y: self.y,
            tile_type: ttype,
            dice: self.dice_value,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Port {
    pub port_type: String,
    pub port_building: Vec<PortBuilding>,
    pub flipped: bool
}

impl Port {
    fn to_event(&self) -> Result<PortEvent, ExternalExecutionError> {
        let ptype = match port_type_from_string(&self.port_type) {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        Ok(PortEvent{ 
            flipped: self.flipped, 
            port_type: ptype, 
            buildings: [
                self.port_building[0].y * 100 + self.port_building[0].x, 
                self.port_building[1].y * 100 + self.port_building[1].x] 
            }
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct PortBuilding {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Building {
    pub building_type: String,
    pub color: String,
    pub x: i32,
    pub y: i32
}


impl Building{
    fn to_event(self) ->  Result<BuildingEvent, ExternalExecutionError>{
        let btype = match buildingtype_from_string(self.building_type){
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        Ok(BuildingEvent {
            building_type: btype,
            x: self.x,
            y: self.y,
        })
    }
}

fn buildingtype_from_string(color: String) -> Result<BuildingType, ExternalExecutionError>{
    match color.to_uppercase().as_str() {
        "EMPTY" => Ok(BuildingType::EMPTY),
        "SETTELMENT" => Ok(BuildingType::SETTELMENT),
        "TOWN" => Ok(BuildingType::TOWN),
        _ => {
            return Err(ExternalExecutionError {
                status_code: StatusCode::BAD_GATEWAY,
                message: "building type is missing or not found".to_string(),
                step: "parse building color".to_string(),
            })
        }
    }
}


fn tile_type_from_string(tile_type: &String) -> Result<TileType, ExternalExecutionError> {
    match tile_type.to_uppercase().as_str() {
        "HILLS" => Ok(TileType::HILLS),
        "FOREST" => Ok(TileType::FOREST),
        "MOUNTAINS" => Ok(TileType::MOUNTAINS),
        "FIELD" => Ok(TileType::FIELD),
        "PASTURE" => Ok(TileType::PASTURE),
        "DESSERT" => Ok(TileType::DESSERT),
        "WATER" => Ok(TileType::WATER),
        _ => {
            return Err(ExternalExecutionError {
                status_code: StatusCode::BAD_GATEWAY,
                message: "Color is missing or not found".to_string(),
                step: "parse player color".to_string(),
            })
        }
    } 
}

fn port_type_from_string(port_type: &String) -> Result<PortType, ExternalExecutionError> {
    match port_type.to_uppercase().as_str() {
        "ANY" => Ok(PortType::ANY),
        "WOOL" => Ok(PortType::WOOL),
        "LUMBER" => Ok(PortType::LUMBER),
        "GRAIN" => Ok(PortType::GRAIN),
        "ORE" => Ok(PortType::ORE),
        "BRICKS" => Ok(PortType::BRICKS),
        _ => {
            return Err(ExternalExecutionError {
                status_code: StatusCode::BAD_GATEWAY,
                message: "Color is missing or not found".to_string(),
                step: "parse player color".to_string(),
            })
        }
    } 
}