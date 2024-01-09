use std::rc::Rc;

use self::{building::Building, tiles::Tile, street::Street};



pub mod tiles;
pub mod street;
pub mod building;

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