use axum::http::StatusCode;
use game::{eventque::start::BuildingEvent, game::BuildingType};
use serde::{Deserialize, Serialize};

use crate::error::ExternalExecutionError;

#[derive(Serialize, Deserialize)]
pub struct Building {
    pub building_type: String,
    pub color: String,
    pub x: i32,
    pub y: i32,
}

impl Building {
    pub fn to_event(self) -> Result<BuildingEvent, ExternalExecutionError> {
        let btype = match buildingtype_from_string(self.building_type) {
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

fn buildingtype_from_string(color: String) -> Result<BuildingType, ExternalExecutionError> {
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
