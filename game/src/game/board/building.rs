use crate::{
    eventque::event::ExecuteError,
    game::{BuildingType, Color},
};
use std::rc::Rc;

use super::ports::Port;

#[derive(Debug)]
pub struct Building {
    pub idx: i32,
    pub corr_x: u8,
    pub corr_y: u8,
    pub building_type: BuildingType,
    pub color: Color,
    pub port: Option<Rc<Port>>,
}

pub fn create_buildings(dims: &Vec<u8>, port: &[Rc<Port>]) -> Vec<Rc<Building>> {
    let max_columns = dims.iter().max().unwrap();
    let first_half = dims.iter().position(|x| x == max_columns).unwrap();

    let updatad_dims = [
        &dims[0..first_half],
        &[*max_columns],
        &dims[first_half..dims.len()],
    ]
    .concat();

    let mut buildings = Vec::new();

    for (row, column) in updatad_dims.iter().enumerate() {
        let row_shift = max_columns - column;

        for i in 0..(column * 2) + 1 {
            let idx = (row * 100) as i32 + (row_shift + i) as i32;
            let port: Option<Rc<Port>> = port.iter().find(|x| x.building_idx == idx).map(Rc::clone);
            let b = Building {
                port,
                idx,
                corr_y: row as u8,
                corr_x: row_shift + i,
                building_type: BuildingType::EMPTY,
                color: Color::DEFAULT,
            };
            buildings.push(Rc::new(b));
        }
    }

    buildings
}

pub fn verify_buldings(
    buildings: &[Rc<Building>],
    target_coordinates: Vec<(i32, i32)>,
) -> Result<(), ExecuteError> {
    for (x, y) in target_coordinates {
        let idx = y * 100 + x;
        let building = buildings.iter().find(|x| x.idx == idx);
        match building {
            Some(_) => (),
            None => {
                return Err(ExecuteError {
                    message: "can not find bulding".to_string(),
                    step: "verify buildings".to_string(),
                })
            }
        };
    }

    Ok(())
}
