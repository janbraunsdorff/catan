use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use game::{
    self,
    eventque::start::{FillBoardEvent, Robber, TileEvent, PortEvent, BuildingEvent},
};

use crate::{error::ExternalExecutionError, routes::{state::StateResponse, commen_models::{tile::Tile, port::Port, building::Building}}};
use serde::{Deserialize, Serialize};




pub async fn fill(
    Path(id): Path<String>,
    Json(payload): Json<FillBoardRequest>,
) -> Result<impl IntoResponse, ExternalExecutionError> {

    let x: Result<Vec<BuildingEvent>, _> = payload.buildings.into_iter().map(|x| x.to_event()).collect();
    let buildings = match x {
        Ok(val) => val,
        Err(err) =>return Err(err),
    };

    let x: Result<Vec<TileEvent>, _> = payload.tiles.iter().map(|t| t.to_event()).collect();
    let tiles = match x {
        Ok(val) => val,
        Err(err) => return Err(err),
    };

    let x: Result<Vec<PortEvent>, ExternalExecutionError> = payload.ports.iter().map(|x| x.to_event()).collect();
    let ports = match x {
        Ok(val) => val,
        Err(err) => return Err(err),
    };


    let new_event = FillBoardEvent {
        tiles: tiles,
        format: payload.format,
        ports: ports,
        bulding: buildings,
        robber: Robber { x: -1, y: -1 },
    };
    let res = game::load_and_execute(id.as_str(), new_event, -1);

    let game = match res {
        Ok(val) => val,
        Err(err) => {
            return Err(ExternalExecutionError {
                step: err.step,
                message: err.message,
                status_code: StatusCode::BAD_REQUEST,
            })
        }
    };
    Ok((StatusCode::OK, Json(StateResponse::from(game))))
}

#[derive(Serialize, Deserialize)]
pub struct FillBoardRequest {
    pub tiles: Vec<Tile>,
    pub format: Vec<u8>,
    pub ports: Vec<Port>,
    pub buildings: Vec<Building>
}
