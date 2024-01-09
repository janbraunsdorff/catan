use std::{env, cmp};

use eventque as eq;
use eventque::event::{Event, ExecuteError, UndoError};
use game::Game;
use serde::{Deserialize, Serialize};

mod _board;
pub mod eventque;
mod game;


fn get_storage() -> String {
    env::var("event_store").unwrap_or("/home/jan/projects/rust-catan/.storage".to_string())
}

pub fn execute<'a>(
    game_idx: String,
    new_event: impl Event + Deserialize<'a> + Serialize + Clone,
    limit: i32
) -> Result<Game, ExecuteError> {
    // load eventstore
    let path = vec![
        get_storage(),
        "/".to_string(),
        game_idx,
        ".jsonl".to_string(),
    ]
    .join("");
    let events_ = eq::load_events(&path);

    // append current event to list
    let mut events = match events_ {
        Ok(value) => value,
        Err(err) => return Err(err),
    };
    events.push(Box::new(new_event.clone()));

    let max_index = if limit > 0 {cmp::min(limit as usize, events.len())} else {events.len()};

    // execute
    let mut game = Game::new();
    for event in events.iter().take(max_index){
        game = match event.execute(game) {
            Ok(val) => val,
            Err(err) => {
                return Err(ExecuteError {
                    step: "event execution".to_string(),
                    message: err.message,
                })
            }
        };
    }

    // store last
    let x = eq::store_event(path, new_event.clone());
    match x {
        Ok(_) => (),
        Err(err) => return Err(err),
    }

    // return Error
    Ok(game)
}

pub fn undo() -> Result<(), UndoError> {
    Ok(())
}
