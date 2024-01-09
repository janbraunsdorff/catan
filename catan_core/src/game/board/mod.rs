use std::rc::Rc;

use self::{building::Building, street::Street, tiles::Tile};

pub mod building;
pub mod street;
pub mod tiles;
pub mod ports;

#[derive(Debug)]
pub struct Board {
    pub buildings: Vec<Rc<Building>>,

    pub tiles: Vec<Tile>,
    pub streets: Vec<Street>,
}

impl Board {
    pub fn new(tiles: Vec<Tile>, buildings: Vec<Rc<Building>>, streets: Vec<Street>) -> Board {
        Board {
            tiles: tiles,
            buildings: buildings,
            streets: streets,
        }
    }
}
