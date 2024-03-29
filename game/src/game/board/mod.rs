use std::rc::Rc;

use self::{building::Building, ports::Port, street::Street, tiles::Tile};

pub mod building;
pub mod ports;
pub mod street;
pub mod tiles;

#[derive(Debug)]
pub struct Board {
    pub buildings: Vec<Rc<Building>>,
    pub ports: Vec<Rc<Port>>,
    pub tiles: Vec<Tile>,
    pub streets: Vec<Street>,
}

impl Board {
    pub fn new(
        tiles: Vec<Tile>,
        buildings: Vec<Rc<Building>>,
        streets: Vec<Street>,
        ports: Vec<Rc<Port>>,
    ) -> Board {
        Board {
            tiles,
            buildings,
            streets,
            ports,
        }
    }
}
