use std::rc::Rc;

#[derive(Debug)]
pub struct Building {
    idx: u8,
    pub corr_x: u8,
    pub corr_y: u8,
}


pub fn create_buildings(dims: &Vec<u8>) -> Vec<Rc<Building>> {
    let max_columns = dims.iter().max().unwrap();
    let first_half = dims.iter().position(|x| x == max_columns).unwrap();

    let updatad_dims = [
        &dims[0..first_half], 
        &vec![*max_columns], 
        &dims[first_half..dims.len()]
    ].concat();



    let mut buildings = Vec::new();
    let mut idx_counter = 0;

    for row in 0..updatad_dims.len() {
        let row_shift = max_columns - updatad_dims[row];

        for i in 0..(updatad_dims[row]*2)+1 {
            let b = Building {
                idx:idx_counter,
                corr_y: row as u8,
                corr_x: row_shift + i,
            };
            buildings.push(Rc::new(b));
            idx_counter += 1;
        }

    }


    buildings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_buildings() {
        let buildings = create_buildings(&vec![1,2,1]);

        for i in 0..3 {
            assert_eq!(buildings[i].corr_x, i as u8 + 1, "checking for 0-3 / {}", i);
            assert_eq!(buildings[i].corr_y, 0, "checking for 0-3 y")
        }

        for i in 3..8 {
            assert_eq!(buildings[i].corr_x, i as u8 -3, "checking for 5-8 / {}", i);
            assert_eq!(buildings[i].corr_y, 1, "checking for 3-8 y")
        }

        for i in 8..13 {
            assert_eq!(buildings[i].corr_x, i as u8 -8, "checking for 8-13 / {}", i);
            assert_eq!(buildings[i].corr_y, 2, "checking for 3-8 y")
        }

        for i in 13..16 {
            assert_eq!(buildings[i].corr_x, i as u8 -12, "checking for 13-16 / {}", i);
            assert_eq!(buildings[i].corr_y, 3, "checking for 3-8 y")
        }
    }
}