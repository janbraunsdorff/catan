use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use game::{
    self,
    eventque::start::CreateGameEvent,
    game::{Color, Player},
};

use crate::{error::ExternalExecutionError, routes::state::StateResponse};
use serde::{Deserialize, Serialize};



pub async fn new(
    Path(id): Path<String>,
    Json(payload): Json<CreateNewGameRequest>,
) -> Result<impl IntoResponse, ExternalExecutionError> {
    let npc = match PlayerRequest::to_players(payload.npc, true) {
        Ok(x) => x,
        Err(err) => return Err(err),
    };

    let player = match PlayerRequest::to_players(payload.player, false) {
        Ok(x) => x,
        Err(err) => return Err(err),
    };

    let new_event = CreateGameEvent {
        npc: npc,
        player: player,
        extentiosns: payload.extentiosns,
    };

    let res = game::load_and_execute(id.as_str(), new_event, -1);

    let game = match res {
        Ok(val) => val,
        Err(err) => {
            return Err(ExternalExecutionError {
                status_code: StatusCode::BAD_REQUEST,
                step: err.step,
                message: err.message,
            })
        }
    };
    Ok((StatusCode::OK, Json(StateResponse::from(game))))
}






#[derive(Serialize, Deserialize)]
pub struct CreateNewGameRequest {
    pub npc: Vec<PlayerRequest>,
    pub player: Vec<PlayerRequest>,
    pub extentiosns: Vec<String>,
}


#[derive(Serialize, Deserialize)]
pub struct PlayerRequest {
    pub name: String,
    pub color: String,
}

impl PlayerRequest {
    fn to_player(&self, is_npc: bool) -> Result<Player, ExternalExecutionError> {
        let color = match color_from_string(&self.color) {
            Ok(val) => val,
            Err(err) => return Err(err),
        };

        Ok(Player {
            color: color,
            name: self.name.clone(),
            npc: is_npc,
        })
    }

    fn to_players(players: Vec<Self>, is_npc: bool) -> Result<Vec<Player>, ExternalExecutionError> {
        let player: Result<Vec<Player>, ExternalExecutionError> =
            players.into_iter().map(|x| x.to_player(is_npc)).collect();

        player
    }
}

fn color_from_string(color: &String) -> Result<Color, ExternalExecutionError>{
    match color.to_uppercase().as_str() {
        "RED" => Ok(Color::RED),
        "BLUE" => Ok(Color::BLUE),
        "ORANGE" => Ok(Color::ORANGE),
        "WHITE" => Ok(Color::WHITHE),
        _ => {
            return Err(ExternalExecutionError {
                status_code: StatusCode::BAD_GATEWAY,
                message: "Color is missing or not found".to_string(),
                step: "parse player color".to_string(),
            })
        }
    }
}

