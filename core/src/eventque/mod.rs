
use std::{fs::OpenOptions, io::{BufReader, BufRead, Write}};

use serde::Serialize;

use self::{event::{ExecuteError, Event}, start::{CreateGameEvent, FillBoardEvent}};
use self::event as ev;

pub mod start;
pub mod event;

pub fn load_events(path: &String) -> Result<Vec<Box<dyn Event>>, ExecuteError>{
    let res_file_open = OpenOptions::new().read(true).open(&path);

    let file = match  res_file_open {
        Ok(file) => file,
        Err(err) => return Err(ExecuteError{step: "open file".to_string(), message: err.to_string()}),
    };

    let reader = BufReader::new(file);
    let mut events = Vec::new();
    for line in reader.lines() {
        let parse_event = match line {
            Ok(val) => parse_line(val.as_str()),
            Err(err) => return Err(ExecuteError{step: "read line".to_string(), message: err.to_string()}),
        };

        let event = match parse_event {
            Ok(value) => value,
            Err(err) => return Err(err),
        };
        events.push(event);
    };

    Ok(events)

}

fn parse_line(line: &str) -> Result<Box<dyn Event>, ExecuteError>{
    let parts: Vec<&str> = line.split(":").collect();
    let event_type = parts[0];

    let data = parts[1..].join("");




    let mut event_missing_error = "could not find event ".to_string();
    event_missing_error.push_str(event_type);

    let mut v: Vec<Box<dyn Event>> = Vec::new();

    let d1  = match  ev::from_str::<CreateGameEvent>(data.as_str()){
        Ok(val) => val,
        Err(err) => todo!(),
    };

    v.push(d1);


    let x: Result<Box<dyn Event>, ExecuteError> = match event_type {
        "CreateGame" => ev::from_str::<CreateGameEvent>(data.as_str()),
        "FillBoard" => ev::from_str::<FillBoardEvent>(data.as_str()), 
        _ => return Err(ExecuteError{step: "parse line".to_string(), message: event_missing_error})
    };

    match x {
        Ok(val) => Ok(val),
        Err(err) => Err(err),
    }

}





pub fn store_event(path: String, event: impl Serialize) -> Result<(), ExecuteError>{
    let res_file_open = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path);

    let mut queue = match  res_file_open {
        Ok(file) => file,
        Err(err) => return  Err(ExecuteError{step: "saving event open file".to_string(), message: err.to_string()}),
    };


    let mut json_value = match serde_json::to_string(&event) {
        Ok(val) => val,
        Err(err) => return Err(ExecuteError{step: "serilize new event".to_string(), message: err.to_string()}),
    };

    json_value.push('\n');


    let res_write = queue.write_all(json_value.as_bytes());
    match res_write {
        Ok(_) => (),
        Err(err) => return  Err(ExecuteError{step: "saving event to file".to_string(), message: err.to_string()}),
    }

    let res_lf = queue.write("\n".as_bytes());
    match res_lf {
        Ok(_) => (),
        Err(err) => return  Err(ExecuteError{step: "saving new line to file".to_string(), message: err.to_string()}),
    }


     Ok(())
}

