
use crate::*;
use crate::constants::{self, MAP_DIM_X, MAP_DIM_Y};

pub fn createBlankGrid() -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = vec![];
    for y in 0..MAP_DIM_Y {
        grid.push(vec![]);
        for _x in 0..MAP_DIM_X {
            grid[y].push(false);
        }
    }
    return grid;
}