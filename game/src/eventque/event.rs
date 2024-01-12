use std::io::BufReader;

use crate::game::Game;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct ExecuteError {
    pub message: String,
    pub step: String,
}
pub struct UndoError {}

pub trait Event {
    fn execute(&self, game: Game) -> Result<Game, ExecuteError>;
    fn undo(&self) -> Result<(), UndoError>;
    fn get_name(&self) -> String;
}

pub fn from_str<'a, T>(data: &str) -> Result<Box<dyn Event + 'a>, ExecuteError>
where
    T: DeserializeOwned + Event + 'a,
{
    let buf_reader = BufReader::new(data.as_bytes());
    let x: Result<T, serde_json::Error> = serde_json::from_reader(buf_reader);

    match x {
        Ok(val) => Ok(Box::new(val)),
        Err(err) => Err(ExecuteError {
            message: err.to_string(),
            step: "parse from string".to_string(),
        }),
    }
}

pub fn from_trait<T: Event>(t: T) -> Box<T> {
    Box::new(t)
}
