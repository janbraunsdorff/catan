use crate::board::tiles::{create_tiles, TileType};
use crate::board::building::create_buildings;
use crate::board::street::create_streets;
use crate::board::field::Board;


mod field;
mod tiles;
mod street;
mod building;


struct BoardBuilder{
    dims: Vec<u8>,
    current_dim: usize,
    dice_values: Vec<u8>,
    kinds: Vec<TileType>,
    robber_x: u8,
    robber_y: u8,
}

impl BoardBuilder {
    fn from_scratch() -> BoardBuilder{
        BoardBuilder { dims: vec![0], current_dim: 0, dice_values: Vec::new(), kinds: Vec::new(), robber_x: 0, robber_y: 0 }
    }

    fn with_new_line(mut self) -> BoardBuilder {
        self.current_dim += 1;
        self.dims.push(0);
        self
    }

    fn with_tile(mut self, dice_value: u8, kind: TileType) -> BoardBuilder {
        self.dims[self.current_dim] += 1;
        self.dice_values.push(dice_value);
        self.kinds.push(kind);
        self
    }

    fn with_roober(mut self, x: u8, y: u8) -> BoardBuilder{
        self.robber_x = x;
        self.robber_y = y;
        self
    }

    fn build(&self) -> Board {
        let buildings = create_buildings(&self.dims);
        let tiles = create_tiles(&self.dims, &buildings, &self.dice_values, &self.kinds, &self.robber_x, &self.robber_y);
        let streets = create_streets(&self.dims, &buildings);
        Board::new(tiles, buildings, streets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_1x2x1_tile() {
        let board = BoardBuilder::from_scratch()
            .with_tile(12, TileType::Dessert)
            .with_new_line()
            .with_tile(12, TileType::Dessert)
            .with_tile(12, TileType::Dessert)
            .with_new_line()
            .with_tile(12, TileType::Dessert)
            .with_roober(2,0)
            .build();


        assert_eq!(board.tiles.len(), 4);
        assert_eq!(board.tiles[0].get_coordiante(), (2,0));
        assert_eq!(board.tiles[1].get_coordiante(), (1,1));
        assert_eq!(board.tiles[2].get_coordiante(), (3,1));
        assert_eq!(board.tiles[3].get_coordiante(), (2,2));


        for i in 0..3 {
            assert_eq!(board.buildings[i].corr_x, i as u8 + 1, "checking for 0-3 / {}", i);
            assert_eq!(board.buildings[i].corr_y, 0, "checking for 0-3 y")
        }

        for i in 3..8 {
            assert_eq!(board.buildings[i].corr_x, i as u8 -3, "checking for 5-8 / {}", i);
            assert_eq!(board.buildings[i].corr_y, 1, "checking for 3-8 y")
        }

        for i in 8..13 {
            assert_eq!(board.buildings[i].corr_x, i as u8 -8, "checking for 8-13 / {}", i);
            assert_eq!(board.buildings[i].corr_y, 2, "checking for 3-8 y")
        }

        for i in 13..16 {
            assert_eq!(board.buildings[i].corr_x, i as u8 -12, "checking for 13-16 / {}", i);
            assert_eq!(board.buildings[i].corr_y, 3, "checking for 3-8 y")
        }

        assert_eq!(board.tiles[0].robber(), true);


    }
}