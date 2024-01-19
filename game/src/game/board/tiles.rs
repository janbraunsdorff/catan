use std::rc::Rc;

use crate::eventque::event::ExecuteError;
use crate::game::{board::building::Building, TileType};

#[derive(Debug)]
pub struct Tile {
    idx: i32,
    pub corr_x: u8,
    pub corr_y: u8,
    pub buildings: Vec<Rc<Building>>,
    pub dice: u8,
    pub kind: TileType,
    pub has_robber: bool,
}

pub fn create_tiles(
    dims: &Vec<u8>,
    buildings: &[Rc<Building>],
    dice_values: &[u8],
    kinds: &[TileType],
    robber_x: i32,
    robber_y: i32,
) -> Vec<Tile> {
    let mut tiles = Vec::new();
    let mut idx_counter = 0;
    // 0, 1 => 0 1, 1 1, 2 1, 0 2, 1 2, 2 2

    for row in 0..dims.len() {
        let row_shift = dims.iter().max().unwrap() - dims[row];

        for column in 0..dims[row] {
            let x = column + row_shift;
            let y = row as u8;
            let t = Tile {
                idx: y as i32 * 100 + x as i32,
                corr_x: x,
                corr_y: y,
                buildings: vec![
                    Rc::clone(
                        buildings
                            .iter()
                            .find(|b| b.corr_x == x && b.corr_y == y)
                            .unwrap(),
                    ),
                    Rc::clone(
                        buildings
                            .iter()
                            .find(|b| b.corr_x == x + 1 && b.corr_y == y)
                            .unwrap(),
                    ),
                    Rc::clone(
                        buildings
                            .iter()
                            .find(|b| b.corr_x == x + 2 && b.corr_y == y)
                            .unwrap(),
                    ),
                    Rc::clone(
                        buildings
                            .iter()
                            .find(|b| b.corr_x == x && b.corr_y == y + 1)
                            .unwrap(),
                    ),
                    Rc::clone(
                        buildings
                            .iter()
                            .find(|b| b.corr_x == x + 1 && b.corr_y == y + 1)
                            .unwrap(),
                    ),
                    Rc::clone(
                        buildings
                            .iter()
                            .find(|b| b.corr_x == x + 2 && b.corr_y == y + 1)
                            .unwrap(),
                    ),
                ],
                dice: dice_values[idx_counter as usize],
                kind: kinds[idx_counter as usize],
                has_robber: x as i32 == robber_x && y as i32 == robber_y,
            };
            tiles.push(t);
            idx_counter += 1;
        }
    }
    tiles
}

pub fn verify_tiles(
    tiles: &[Tile],
    target_coordinates: Vec<(i32, i32)>,
) -> Result<(), ExecuteError> {
    for (x, y) in target_coordinates {
        let idx = y * 100 + x;
        let building = tiles.iter().find(|x| x.idx == idx);
        match building {
            Some(_) => (),
            None => {
                return Err(ExecuteError {
                    message: "can not find tile".to_string(),
                    step: "verify tiles".to_string(),
                })
            }
        };
    }

    Ok(())
}
