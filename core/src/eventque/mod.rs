use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::Path,
};

use serde::Serialize;

use self::event as ev;
use self::{
    event::{Event, ExecuteError},
    start::{CreateGameEvent, FillBoardEvent},
};

pub mod event;
pub mod start;

pub fn load_events(path: &String) -> Result<Vec<Box<dyn Event>>, ExecuteError> {
    if !Path::new(path).exists() {
        return Ok(vec![]);
    }

    let res_file_open = OpenOptions::new().read(true).open(&path);

    let file = match res_file_open {
        Ok(file) => file,
        Err(err) => {
            return Err(ExecuteError {
                step: "open file".to_string(),
                message: err.to_string(),
            })
        }
    };

    let reader = BufReader::new(file);
    let mut events = Vec::new();
    for line in reader.lines() {
        let line_data = match line {
            Ok(val) => val,
            Err(err) => {
                return Err(ExecuteError {
                    step: "read line".to_string(),
                    message: err.to_string(),
                })
            }
        };

        if line_data.len() == 0 {
            break;
        }

        let event = match parse_line(&line_data) {
            Ok(val) => val,
            Err(err) => return Err(err),
        };

        events.push(event);
    }

    Ok(events)
}

fn parse_line(line: &str) -> Result<Box<dyn Event>, ExecuteError> {
    let parts: Vec<&str> = line.split(":").collect();
    let event_type = parts[0];

    let data = parts[1..].join(":");

    let mut event_missing_error = "could not find event ".to_string();
    event_missing_error.push_str(event_type);

    let x: Result<Box<dyn Event>, ExecuteError> = match event_type {
        "CreateGameEvent" => ev::from_str::<CreateGameEvent>(data.as_str()),
        "FillBoardEvent" => ev::from_str::<FillBoardEvent>(data.as_str()),
        _ => {
            return Err(ExecuteError {
                step: "parse line".to_string(),
                message: event_missing_error,
            })
        }
    };

    match x {
        Ok(val) => Ok(val),
        Err(err) => Err(err),
    }
}

pub fn store_event(path: String, event: impl Event + Serialize) -> Result<(), ExecuteError> {
    let res_file_open = OpenOptions::new().create(true).append(true).open(&path);

    let mut queue = match res_file_open {
        Ok(file) => file,
        Err(err) => {
            return Err(ExecuteError {
                step: "saving event open file".to_string(),
                message: err.to_string(),
            })
        }
    };

    let json_value = match serde_json::to_string(&event) {
        Ok(val) => val,
        Err(err) => {
            return Err(ExecuteError {
                step: "serilize new event".to_string(),
                message: err.to_string(),
            })
        }
    };

    let mut serilisation = event.get_name();
    serilisation.push_str(&json_value);

    let res_write = queue.write_all(serilisation.as_bytes());
    match res_write {
        Ok(_) => (),
        Err(err) => {
            return Err(ExecuteError {
                step: "saving event to file".to_string(),
                message: err.to_string(),
            })
        }
    }

    let res_lf = queue.write("\n".as_bytes());
    match res_lf {
        Ok(_) => (),
        Err(err) => {
            return Err(ExecuteError {
                step: "saving new line to file".to_string(),
                message: err.to_string(),
            })
        }
    }

    Ok(())
}
