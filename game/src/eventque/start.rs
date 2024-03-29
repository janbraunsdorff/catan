use std::rc::Rc;

use serde::{Deserialize, Serialize};

use super::event::{Event, ExecuteError, UndoError};
use crate::game::board::building::{create_buildings, verify_buldings};
use crate::game::board::ports::Port;
use crate::game::board::street::create_streets;
use crate::game::board::tiles::{create_tiles, verify_tiles};
use crate::game::board::Board;
use crate::game::{BuildingType, Game, Player, PortType, TileType};

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateGameEvent {
    pub npc: Vec<Player>,
    pub player: Vec<Player>,
    pub extentiosns: Vec<String>,
}

impl Event for CreateGameEvent {
    fn execute(&self, mut game: Game) -> Result<Game, ExecuteError> {
        let pc = self.player.to_owned();
        let npc = self.npc.to_owned();
        game.players = [pc, npc].concat();
        game.extenstions = self.extentiosns.to_owned();
        Ok(game)
    }

    fn undo(&self) -> Result<(), UndoError> {
        todo!()
    }

    fn get_name(&self) -> String {
        "CreateGameEvent:".to_string()
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct TileEvent {
    pub idx: i32,
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
    pub dice: u8,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct PortEvent {
    pub port_type: PortType,
    pub buildings: [i32; 2],
    pub flipped: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BuildingEvent {
    pub building_type: BuildingType,
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Robber {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FillBoardEvent {
    pub tiles: Vec<TileEvent>,
    pub format: Vec<u8>,
    pub ports: Vec<PortEvent>,
    pub bulding: Vec<BuildingEvent>,
    pub robber: Robber,
}

impl Event for FillBoardEvent {
    fn execute(&self, mut game: Game) -> Result<Game, ExecuteError> {
        let ports: Vec<Rc<Port>> = self
            .ports
            .iter()
            .flat_map(|x| {
                vec![
                    Port {
                        port_type: x.port_type,
                        building_idx: x.buildings[0],
                    },
                    Port {
                        port_type: x.port_type,
                        building_idx: x.buildings[1],
                    },
                ]
            })
            .map(Rc::new)
            .collect();
        // create Buildings
        let buildings = create_buildings(&self.format, &ports);
        let target_corr = self.bulding.iter().map(|x| (x.x, x.y)).collect();
        let res_create_bulding = verify_buldings(&buildings, target_corr);
        match res_create_bulding {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        let dices: Vec<u8> = self.tiles.iter().map(|x| x.dice).collect();
        let tiles_type: Vec<TileType> = self.tiles.iter().map(|x| x.tile_type).collect();

        let tiles = create_tiles(
            &self.format,
            &buildings,
            dices.as_slice(),
            tiles_type.as_slice(),
            self.robber.x,
            self.robber.y,
        );
        let target_tile_corr = self.tiles.iter().map(|x| (x.x, x.y)).collect();
        let res_create_tiles = verify_tiles(&tiles, target_tile_corr);
        match res_create_tiles {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        // create Streets
        let streets = create_streets(&self.format, &buildings);

        game.board = Some(Board::new(tiles, buildings, streets, ports));
        Ok(game)
    }

    fn undo(&self) -> Result<(), UndoError> {
        todo!()
    }

    fn get_name(&self) -> String {
        "FillBoardEvent:".to_string()
    }
}
