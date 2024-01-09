use core::execute;
use core::eventque::start::CreateGameEvent;

#[test]
fn test_create_game() {
    let event = CreateGameEvent {
        npc: vec![],
        player: vec![],
        extenstions: vec![],
    };
    let ex_result = execute("new_game".to_string(), event, -1);

    let res = match ex_result {
        Ok(val) => val,
        Err(err) => panic!("{:?}", err),
    };

}