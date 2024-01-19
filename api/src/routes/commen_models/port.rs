use axum::http::StatusCode;
use game::{eventque::start::PortEvent, game::PortType};
use serde::{Deserialize, Serialize};

use crate::error::ExternalExecutionError;

#[derive(Serialize, Deserialize)]
pub struct Port {
    pub port_type: String,
    pub port_building: Vec<PortBuilding>,
    pub flipped: bool,
}
#[derive(Serialize, Deserialize)]
pub struct PortBuilding {
    pub x: i32,
    pub y: i32,
}

impl Port {
    pub fn to_event(&self) -> Result<PortEvent, ExternalExecutionError> {
        let ptype = match port_type_from_string(&self.port_type) {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        Ok(PortEvent {
            flipped: self.flipped,
            port_type: ptype,
            buildings: [
                self.port_building[0].y * 100 + self.port_building[0].x,
                self.port_building[1].y * 100 + self.port_building[1].x,
            ],
        })
    }
}

fn port_type_from_string(port_type: &str) -> Result<PortType, ExternalExecutionError> {
    match port_type.to_uppercase().as_str() {
        "ANY" => Ok(PortType::ANY),
        "WOOL" => Ok(PortType::WOOL),
        "LUMBER" => Ok(PortType::LUMBER),
        "GRAIN" => Ok(PortType::GRAIN),
        "ORE" => Ok(PortType::ORE),
        "BRICKS" => Ok(PortType::BRICKS),
        _ => Err(ExternalExecutionError {
            status_code: StatusCode::BAD_GATEWAY,
            message: "Color is missing or not found".to_string(),
            step: "parse player color".to_string(),
        }),
    }
}
