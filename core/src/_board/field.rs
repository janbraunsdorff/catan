use crate::_board::building::Building;
use crate::_board::street::Street;
use crate::_board::tiles::Tile;
use std::rc::Rc;

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
