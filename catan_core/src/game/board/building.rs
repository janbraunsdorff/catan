use crate::{eventque::event::ExecuteError, game::{BuildingType, Color}};
use std::rc::Rc;

use super::ports::Port;

#[derive(Debug)]
pub struct Building {
    pub idx: i32,
    pub corr_x: u8,
    pub corr_y: u8,
    pub building_type: BuildingType,
    pub color: Color,
    pub port: Option<Rc<Port>>
}

pub fn create_buildings(dims: &Vec<u8>, port: &Vec<Rc<Port>>) -> Vec<Rc<Building>> {
    let max_columns = dims.iter().max().unwrap();
    let first_half = dims.iter().position(|x| x == max_columns).unwrap();

    let updatad_dims = [
        &dims[0..first_half],
        &vec![*max_columns],
        &dims[first_half..dims.len()],
    ]
    .concat();

    let mut buildings = Vec::new();

    for row in 0..updatad_dims.len() {
        let row_shift = max_columns - updatad_dims[row];

        for i in 0..(updatad_dims[row] * 2) + 1 {
            let idx = (row * 100) as i32 + (row_shift + i) as i32;
            let port: Option<Rc<Port>> =  match port.into_iter().filter(|x| x.building_idx == idx).next() {
                Some(val) => Some(Rc::clone(val)),
                None => None,
            };
            let b = Building {
                port: port,
                idx: idx,
                corr_y: row as u8,
                corr_x: row_shift + i,
                building_type: BuildingType::EMPTY,
                color: Color::DEFAULT
            };
            buildings.push(Rc::new(b));
        }
    }

    buildings
}

pub fn verify_buldings(
    buildings: &Vec<Rc<Building>>,
    target_coordinates: Vec<(i32, i32)>,
) -> Result<(), ExecuteError> {
    for (x, y) in target_coordinates {
        let idx = y * 100 + x;
        let building = buildings.iter().filter(|x| x.idx == idx).next();
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
