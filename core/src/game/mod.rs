use serde::{Serialize, Deserialize};

use crate::_board::{field::Board, Color};


#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    color: Color,
    name: String,
    npc: bool,
}

pub struct Game {
    board: Option<Board>,
    players: Vec<Player>,
    extenstions: Vec<String>
}

impl Game {
    pub fn new() -> Game {
        Game{ board: None, players: Vec::new(), extenstions: Vec:: new() }
    }
}