use crate::game::board::building::Building;
use std::rc::Rc;

#[derive(Debug)]
pub struct Street {
    pub idx: u8,
    c1: Rc<Building>,
    c2: Rc<Building>,
}

pub fn create_streets(dims: &Vec<u8>, buildings: &Vec<Rc<Building>>) -> Vec<Street> {
    let mut streets = Vec::new();
    let mut idx_counter = 0;

    let max_columns = dims.iter().max().unwrap();
    let first_half = dims.iter().position(|x| x == max_columns).unwrap();

    let updatad_dims = [
        &dims[0..first_half],
        &vec![*max_columns],
        &dims[first_half..dims.len()],
    ]
    .concat();

    for row in 0..updatad_dims.len() {
        let row_shift = max_columns - updatad_dims[row];

        for i in 0..(updatad_dims[row] * 2) {
            let s = Street {
                idx: idx_counter,
                c1: Rc::clone(
                    &buildings
                        .iter()
                        .filter(|b| b.corr_y == row as u8 && b.corr_x == row_shift + i)
                        .next()
                        .unwrap(),
                ),
                c2: Rc::clone(
                    &buildings
                        .iter()
                        .filter(|b| b.corr_y == row as u8 && b.corr_x == row_shift + i + 1)
                        .next()
                        .unwrap(),
                ),
            };
            streets.push(s);
            idx_counter += 1;
        }
    }

    for row in 0..dims.len() {
        let row_shift = max_columns - dims[row];

        for column in 0..dims[row] + 1 {
            let c = row_shift + (column * 2);
            let r = row as u8;

            let s = Street {
                idx: idx_counter,
                c1: Rc::clone(
                    &buildings
                        .iter()
                        .filter(|b| b.corr_y == r && b.corr_x == c)
                        .next()
                        .unwrap(),
                ),
                c2: Rc::clone(
                    &buildings
                        .iter()
                        .filter(|b| b.corr_y == r + 1 && b.corr_x == c)
                        .next()
                        .unwrap(),
                ),
            };
            streets.push(s);
            idx_counter += 1;
        }
    }
    streets
}