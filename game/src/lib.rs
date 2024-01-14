use std::{cmp, env};

use eventque as eq;
use eventque::event::{Event, ExecuteError, UndoError};
use game::Game;
use serde::{Deserialize, Serialize};

pub mod eventque;
pub mod game;

fn get_storage_path() -> String{
    env::var("event_store").unwrap_or("/home/jan/projects/rust-catan/.storage".to_string())
}

fn get_storage(game_idx: &str) -> String {
    let base_path = get_storage_path();
    let path = vec![base_path.as_str(), "/", game_idx, ".jsonl"].join("");
    return path;
}

fn execute<'a>(
    game_idx: &str,
    new_event: impl Event + Deserialize<'a> + Serialize + Clone,
    game: Game,
) -> Result<Game, ExecuteError> {
    let path = get_storage(&game_idx);

    let updated_game = match new_event.execute(game) {
        Ok(val) => val,
        Err(err) => return Err(err),
    };

    // store last
    let store_result = store(path, new_event.clone());

    match store_result {
        Ok(_) => (),
        Err(err) => return Err(err),
    }

    // return Error
    Ok(updated_game)
}

fn store<'a>(
    path: String,
    new_event: impl Event + Deserialize<'a> + Serialize + Clone,
) -> Result<(), ExecuteError> {
    let x = eq::store_event(path, new_event.clone());
    match x {
        Ok(_) => Ok(()),
        Err(err) => return Err(err),
    }
}

pub fn load<'a>(game_idx: &str, limit: i32) -> Result<Game, ExecuteError> {
    let path = get_storage(game_idx);
    let events_ = eq::load_events(&path);

    let events = match events_ {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

    let max_index = if limit > 0 {
        cmp::min(limit as usize, events.len())
    } else {
        events.len()
    };

    // execute
    let mut game = Game::new();
    for event in events.iter().take(max_index) {
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

    Ok(game)
}

pub fn load_and_execute<'a>(
    game_idx: &str,
    new_event: impl Event + Deserialize<'a> + Serialize + Clone,
    limit: i32,
) -> Result<Game, ExecuteError> {
    let game = match load(game_idx, limit) {
        Ok(val) => val,
        Err(err) => return Err(err),
    };

    match execute(game_idx, new_event, game) {
        Ok(val) => Ok(val),
        Err(err) => Err(err),
    }
}

pub fn undo() -> Result<(), UndoError> {
    Ok(())
}
