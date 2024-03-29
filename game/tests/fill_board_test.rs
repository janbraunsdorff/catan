use game::{
    eventque::start::{BuildingEvent, FillBoardEvent, PortEvent, Robber, TileEvent},
    game::{BuildingType, PortType, TileType},
    load, load_and_execute,
};
use std::fs;

#[test]
fn test_fill_board() {
    let event = FillBoardEvent {
        tiles: vec![
            TileEvent {
                idx: 0,
                x: 1,
                y: 0,
                tile_type: TileType::DESSERT,
                dice: 6,
            },
            TileEvent {
                idx: 1,
                x: 0,
                y: 1,
                tile_type: TileType::DESSERT,
                dice: 6,
            },
            TileEvent {
                idx: 2,
                x: 1,
                y: 1,
                tile_type: TileType::DESSERT,
                dice: 6,
            },
            TileEvent {
                idx: 3,
                x: 1,
                y: 2,
                tile_type: TileType::DESSERT,
                dice: 6,
            },
        ],
        format: vec![1, 2, 1],
        ports: vec![PortEvent {
            flipped: false,
            port_type: PortType::ANY,
            buildings: [101, 102],
        }],
        bulding: vec![
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 1,
                y: 0,
            },
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 2,
                y: 0,
            },
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 3,
                y: 0,
            },
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 0,
                y: 1,
            },
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 1,
                y: 1,
            },
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 2,
                y: 1,
            },
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 3,
                y: 1,
            },
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 4,
                y: 1,
            },
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 0,
                y: 2,
            },
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 1,
                y: 2,
            },
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 2,
                y: 2,
            },
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 3,
                y: 2,
            },
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 4,
                y: 2,
            },
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 1,
                y: 3,
            },
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 2,
                y: 3,
            },
            BuildingEvent {
                building_type: BuildingType::EMPTY,
                x: 3,
                y: 3,
            },
        ],
        robber: Robber { x: 1, y: 2 },
    };

    let _ = fs::copy(
        "/home/jan/projects/rust-catan/.storage/002_fill_board_1.jsonl",
        "/home/jan/projects/rust-catan/.storage/fill_board.jsonl",
    );
    let ex_result = load_and_execute("fill_board", event, -1);
    let _ = fs::remove_file("/home/jan/projects/rust-catan/.storage/fill_board.jsonl");

    let res = match ex_result {
        Ok(val) => val,
        Err(err) => panic!("{:?}", err),
    };

    assert!(res.board.is_some());

    let b = res.board.unwrap();
    assert_eq!(b.streets.len(), 19);
    assert_eq!(b.tiles.len(), 4);
    assert_eq!(b.buildings.len(), 16);

    assert!(b
        .buildings
        .iter()
        .find(|x| x.idx == 101)
        .unwrap()
        .port
        .is_some());
    let port = b.buildings.iter().find(|x| x.idx == 101).unwrap();
    let x = match &port.as_ref().port {
        Some(val) => val.port_type,
        None => panic!(""),
    };
    assert_eq!(x, PortType::ANY);

    let port = b.buildings.iter().find(|x| x.idx == 102).unwrap();
    let x = match &port.as_ref().port {
        Some(val) => val.port_type,
        None => panic!(""),
    };
    assert_eq!(x, PortType::ANY);
}

#[test]
fn test_load_board() {
    let ex_result = load("002_fill_board_2", -1);

    let res = match ex_result {
        Ok(val) => val,
        Err(err) => panic!("{:?}", err),
    };

    assert!(res.board.is_some());

    let b = res.board.unwrap();
    assert_eq!(b.streets.len(), 19);
    assert_eq!(b.tiles.len(), 4);
    assert_eq!(b.buildings.len(), 16);
}
