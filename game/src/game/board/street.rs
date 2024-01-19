use crate::game::{board::building::Building, Color};
use std::rc::Rc;

#[derive(Debug)]
pub struct Street {
    pub idx: u8,
    pub color: Color,
    pub c1: Rc<Building>,
    pub c2: Rc<Building>,
}

pub fn create_streets(dims: &Vec<u8>, buildings: &[Rc<Building>]) -> Vec<Street> {
    let mut streets = Vec::new();
    let mut idx_counter = 0;

    let max_columns = dims.iter().max().unwrap();
    let first_half = dims.iter().position(|x| x == max_columns).unwrap();

    let updatad_dims = [
        &dims[0..first_half],
        &[*max_columns],
        &dims[first_half..dims.len()],
    ]
    .concat();

    for (row, column) in updatad_dims.iter().enumerate() {
        let row_shift = max_columns - column;

        for i in 0..(column * 2) {
            let s = Street {
                idx: idx_counter,
                color: Color::DEFAULT,
                c1: Rc::clone(
                    buildings
                        .iter()
                        .find(|b| b.corr_y == row as u8 && b.corr_x == row_shift + i)
                        .unwrap(),
                ),
                c2: Rc::clone(
                    buildings
                        .iter()
                        .find(|b| b.corr_y == row as u8 && b.corr_x == row_shift + i + 1)
                        .unwrap(),
                ),
            };
            streets.push(s);
            idx_counter += 1;
        }
    }

    for (row, columns) in dims.iter().enumerate() {
        let row_shift = max_columns - columns;

        for column in 0..columns + 1 {
            let c = row_shift + (column * 2);
            let r = row as u8;

            let s = Street {
                idx: idx_counter,
                color: Color::DEFAULT,
                c1: Rc::clone(
                    buildings
                        .iter()
                        .find(|b| b.corr_y == r && b.corr_x == c)
                        .unwrap(),
                ),
                c2: Rc::clone(
                    buildings
                        .iter()
                        .find(|b| b.corr_y == r + 1 && b.corr_x == c)
                        .unwrap(),
                ),
            };
            streets.push(s);
            idx_counter += 1;
        }
    }
    streets
}
