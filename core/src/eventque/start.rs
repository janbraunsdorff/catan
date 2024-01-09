use serde::{Deserialize, Serialize};

use crate::game::{Game, Player};

use super::event::{Event, ExecuteError, UndoError};

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateGameEvent {
    pub npc: Vec<Player>,
    pub player: Vec<Player>,
    pub extenstions: Vec<String>,
}

impl Event for CreateGameEvent {
    fn execute(&self, game: Game) -> Result<Game, ExecuteError> {
        Ok(game)
    }

    fn undo(&self) -> Result<(), UndoError> {
        todo!()
    }

    fn get_name(&self) -> String {
        "CreateGameEvent:".to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub struct FillBoardEvent {}

impl Event for FillBoardEvent {
    fn execute(&self, game: Game) -> Result<Game, ExecuteError> {
        Ok(game)
    }

    fn undo(&self) -> Result<(), UndoError> {
        todo!()
    }

    fn get_name(&self) -> String {
        "FillBoardEvent:".to_string()
    }
}
