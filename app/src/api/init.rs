use axum::{Json, response::IntoResponse, http::StatusCode, extract::Path};
use catan_core::{self, eventque::start::{CreateGameEvent, FillBoardEvent, Robber}};

use crate::error::ExternalExecutionError;
use super::model::game::{CreateNewGameRequest, StateResponse, FillBoardRequest};



pub async fn new(Path(id): Path<String>, Json(payload): Json<CreateNewGameRequest>)  ->  Result<impl IntoResponse, ExternalExecutionError> {
    let new_event = CreateGameEvent {
        npc: vec![],
        player: vec![],
        extenstions: vec![],
    };
    let res = catan_core::load_and_execute(id.as_str(), new_event, -1);

    let game = match res {
        Ok(val) => val,
        Err(err) => return Err(ExternalExecutionError{ step: err.step, message: err.message })
    };
    Ok((StatusCode::OK, Json(StateResponse::from(game))))
}


pub async fn fill(Path(id): Path<String>, Json(payload): Json<FillBoardRequest>)  ->  Result<impl IntoResponse, ExternalExecutionError> {
    let new_event = FillBoardEvent {
        tiles: vec![],
        format: vec![],
        ports: vec![],
        bulding: vec![],
        robber: Robber{x: -1, y:-1},
    };
    let res = catan_core::load_and_execute(id.as_str(), new_event, -1);

    let game = match res {
        Ok(val) => val,
        Err(err) => return Err(ExternalExecutionError{ step: err.step, message: err.message })
    };
    Ok((StatusCode::OK, Json(StateResponse::from(game))))
}