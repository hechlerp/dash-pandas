
use crate::*;
/// Imports constants and modules from the `constants` module.
/// 
/// The following constants are imported:
/// - `CELL_SIZE`: Represents the size of each cell in the grid.
/// - `MAP_DIM_X`: Represents the dimension of the map along the X-axis.
/// - `MAP_DIM_Y`: Represents the dimension of the map along the Y-axis.
/// 
/// The `self` import allows access to other items within the `constants` module.
use crate::constants::{self, CELL_SIZE, MAP_DIM_X, MAP_DIM_Y};



pub fn createBlankGrid() -> Vec<Vec<CELLVAL>> {
    let mut grid: Vec<Vec<CELLVAL>> = vec![];
    for y in 0..MAP_DIM_Y {
        grid.push(vec![]);
        for _x in 0..MAP_DIM_X {
            grid[y].push(CELLVAL::Empty);
        }
    }
    return grid;
}

const WALL_COLOR: u32 = 0x96885dFF;

pub fn drawGrid(grid: Vec<Vec<CELLVAL>>) {
    // for y in 0..MAP_DIM_Y {
    //     for x in 0..MAP_DIM_X {
    //         if grid[y][x] {
    //             let xCoord: usize = x * CELL_SIZE;
    //             let yCoord: usize = y * CELL_SIZE;
    //             rect!(x = xCoord, y = yCoord, w = CELL_SIZE, h = CELL_SIZE, color = WALL_COLOR, border_radius = 0);
    //         }
    //     }
    // }
}