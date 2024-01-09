use catan_core::eventque::start::CreateGameEvent;
use catan_core::game::{Color, Player};
use catan_core::{load, load_and_execute};
use std::fs;

#[test]
fn test_create_game_from_event() {
    let event = CreateGameEvent {
        npc: vec![Player {
            color: Color::RED,
            name: "npc1".to_string(),
            npc: true,
        }],
        player: vec![Player {
            color: Color::BLUE,
            name: "pc1".to_string(),
            npc: false,
        }],
        extenstions: vec![],
    };
    let ex_result = load_and_execute("new_game", event, -1);

    let res = match ex_result {
        Ok(val) => val,
        Err(err) => panic!("{:?}", err),
    };

    let _ = fs::remove_file("/home/jan/projects/rust-catan/.storage/new_game.jsonl");
    assert_eq!(res.players.len(), 2);
    assert_eq!(res.players[0].color, Color::BLUE);
    assert_eq!(res.players[1].color, Color::RED);
}

#[test]
fn test_create_game_from_source() {
    let res = match load("001_new_game", -1) {
        Ok(val) => val,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(res.players.len(), 2);
    assert_eq!(res.players[0].color, Color::BLUE);
    assert_eq!(res.players[1].color, Color::RED);
}
