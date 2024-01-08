use serde::{Deserialize, Serialize};

use crate::game::{Game, Player};

use super::event::{Event, ExecuteError, UndoError};

#[derive(Serialize, Deserialize)]
pub struct CreateGameEvent {
    pub npc: Vec<Player>,
    pub player: Vec<Player>,
    pub extenstions: Vec<String>,
}

impl Event for CreateGameEvent {
    fn execute(&self, game: Game) -> Result<Game, ExecuteError> {
        todo!()
    }

    fn undo(&self) -> Result<(), UndoError> {
        todo!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct FillBoardEvent {}

impl Event for FillBoardEvent {
    fn execute(&self, game: Game) -> Result<Game, ExecuteError> {
        todo!()
    }

    fn undo(&self) -> Result<(), UndoError> {
        todo!()
    }
}
