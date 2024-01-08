use serde::{Deserialize, Serialize};
use std::rc::Rc;

use crate::_board::building::Building;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TileType {
    Hills,
    Forest,
    Mountains,
    Fields,
    Pasture,
    Dessert,
    Water,
}

#[derive(Debug)]
pub struct Tile {
    idx: u8,
    corr_x: u8,
    corr_y: u8,
    buildings: Vec<Rc<Building>>,
    dice: u8,
    kind: TileType,
    has_robber: bool,
}

impl Tile {
    pub fn get_coordiante(&self) -> (u8, u8) {
        (self.corr_x, self.corr_y)
    }

    pub fn robber(&self) -> bool {
        self.has_robber
    }
}

pub fn create_tiles(
    dims: &Vec<u8>,
    buildings: &Vec<Rc<Building>>,
    dice_values: &Vec<u8>,
    kinds: &Vec<TileType>,
    robber_x: &u8,
    robber_y: &u8,
) -> Vec<Tile> {
    let mut tiles = Vec::new();
    let mut idx_counter = 0;

    for row in 0..dims.len() {
        let row_shift = dims.iter().max().unwrap() - dims[row] + 1;

        for column in 0..dims[row] {
            let x = row_shift + (column * 2);
            let y = row as u8;
            let t = Tile {
                idx: idx_counter,
                corr_x: x,
                corr_y: y,
                buildings: vec![
                    Rc::clone(
                        &buildings
                            .iter()
                            .filter(|b| b.corr_x == x - 1 && b.corr_y == y)
                            .next()
                            .unwrap(),
                    ),
                    Rc::clone(
                        &buildings
                            .iter()
                            .filter(|b| b.corr_x == x && b.corr_y == y)
                            .next()
                            .unwrap(),
                    ),
                    Rc::clone(
                        &buildings
                            .iter()
                            .filter(|b| b.corr_x == x + 1 && b.corr_y == y)
                            .next()
                            .unwrap(),
                    ),
                    Rc::clone(
                        &buildings
                            .iter()
                            .filter(|b| b.corr_x == x - 1 && b.corr_y == y + 1)
                            .next()
                            .unwrap(),
                    ),
                    Rc::clone(
                        &buildings
                            .iter()
                            .filter(|b| b.corr_x == x && b.corr_y == y + 1)
                            .next()
                            .unwrap(),
                    ),
                    Rc::clone(
                        &buildings
                            .iter()
                            .filter(|b| b.corr_x == x + 1 && b.corr_y == y + 1)
                            .next()
                            .unwrap(),
                    ),
                ],
                dice: dice_values[idx_counter as usize],
                kind: kinds[idx_counter as usize],
                has_robber: x == *robber_x && y == *robber_y,
            };
            tiles.push(t);
            idx_counter += 1;
        }
    }
    tiles
}

#[cfg(test)]
mod tests {
    use crate::_board::building::create_buildings;

    use super::*;

    #[test]
    fn test_create_1x2x1_tile() {
        let dims = vec![1, 2, 1];

        let tiles = create_tiles(
            &vec![1, 2, 1],
            &create_buildings(&dims),
            &vec![1, 2, 3, 4],
            &vec![
                TileType::Dessert,
                TileType::Dessert,
                TileType::Dessert,
                TileType::Dessert,
            ],
            &2,
            &0,
        );

        assert_eq!(tiles.len(), 4);
        assert_eq!(tiles[0].corr_x, 2, "0x");
        assert_eq!(tiles[0].corr_y, 0, "0y");

        assert_eq!(tiles[1].corr_x, 1, "1x");
        assert_eq!(tiles[1].corr_y, 1, "1y");

        assert_eq!(tiles[2].corr_x, 3, "2x");
        assert_eq!(tiles[2].corr_y, 1, "2y");

        assert_eq!(tiles[3].corr_x, 2, "3x");
        assert_eq!(tiles[3].corr_y, 2, "3y");
    }
}
