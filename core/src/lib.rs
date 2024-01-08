use std::env;

use eventque::event::{Event, ExecuteError, UndoError};
use eventque as eq;
use game::Game;
use serde::{Serialize, Deserialize};






mod _board;
mod eventque;
mod game;


fn get_storage() -> String {
    env::var("event_store").unwrap_or("/home/jan/projects/rust-catan/.storage".to_string())
}


fn execute<'a>(game_idx: String, new_event: impl Event+Deserialize<'a>) -> Result<(), ExecuteError> {
    // load eventstore
    let path = vec![get_storage(), "/".to_string(), game_idx, ".jsonl".to_string()].join("");
    let events_ = eq::load_events(&path);
    
    // append current event to list
    let mut events = match events_ {
        Ok(value) => value,
        Err(err) => return Err(err),
    };
    
    events.push(Box::new(new_event));
    
    // execute
    let mut game = Game::new();
    for event in events {
        game = match  event.execute(game){
            Ok(val) => val,
            Err(err) => return Err(ExecuteError{step: "event execution".to_string(), message: err.message}),
        };
    }


    // store last
    // eq::store_event(path, new_event);

    // return Error
    Ok(())
}

fn undo() -> Result<(), UndoError>{
    Ok(())
}