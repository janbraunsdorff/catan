use catan_core::game::{Game, board::Board};
use serde::{Serialize, Deserialize};

use super::Tile;

#[derive(Serialize, Deserialize)]
pub struct StateResponse {
    pub tiles: Vec<Tile>
}

impl StateResponse {
    pub fn from(game: Game) -> StateResponse{
        StateResponse {
            tiles: StateResponse::parse_tiles(game.board),
        }
    }

    fn parse_tiles(board: Option<Board>) -> Vec<Tile>{
        let board = match board {
            Some(val) => val,
            None => return vec![],
        };

        board.tiles.iter()
            .map(|t| Tile{ x: t.corr_x, y: t.corr_y, tile_type: t.kind, dice_value: t.dice })
            .collect()
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateNewGameRequest {
    pub tiles: Vec<Tile>
}

#[derive(Serialize, Deserialize)]
pub struct FillBoardRequest {

}