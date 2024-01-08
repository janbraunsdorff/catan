use serde::{Serialize, Deserialize};
use crate::game::Game;


pub struct ExecuteError {
    pub message: String,
    pub step: String
}
pub struct UndoError {}

pub trait Event {
    fn execute(&self, game: Game) -> Result<Game, ExecuteError>;
    fn undo(&self) -> Result<(), UndoError>;
}


pub struct EventBox{
    pub event: Box<dyn Event>
}



impl EventBox{
    pub fn from_str<'a, X: Event+Deserialize<'a>+ 'static>(data: &'a str) -> Result<EventBox, ExecuteError> {
        let x: Result<X, serde_json::Error> = serde_json::from_str(data);
        match x {
            Ok(val) => Ok(EventBox{event: Box::new(val)}),
            Err(err) => Err(ExecuteError{ message: err.to_string(), step: "parse from string".to_string() }),
        }
    }
}
    
impl EventBox {
    pub fn from_trait(t: impl Event+ 'static) -> EventBox{
        EventBox {
            event: Box::new(t)
        }
    }
}