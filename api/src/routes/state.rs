use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use game::game::{board::Board, Game};
use serde::{Deserialize, Serialize};

use crate::error::ExternalExecutionError;

use super::Tile;

pub async fn state(Path(id): Path<String>) -> Result<impl IntoResponse, ExternalExecutionError> {
    let res = game::load(id.as_str(), -1);
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
pub struct StateResponse {
    pub tiles: Vec<Tile>,
}

impl StateResponse {
    pub fn from(game: Game) -> StateResponse {
        StateResponse {
            tiles: StateResponse::parse_tiles(game.board),
        }
    }

    fn parse_tiles(board: Option<Board>) -> Vec<Tile> {
        let board = match board {
            Some(val) => val,
            None => return vec![],
        };

        board
            .tiles
            .iter()
            .map(|t| Tile {
                x: t.corr_x,
                y: t.corr_y,
                tile_type: t.kind,
                dice_value: t.dice,
            })
            .collect()
    }
}

