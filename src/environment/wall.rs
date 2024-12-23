
use crate::*;
use crate::constants::{DIRECTIONS, CELLVAL, MAP_DIM_X, MAP_DIM_Y};

pub fn isNextStepPosCellAWall(currentX: usize, currentY: usize, direction: &constants::DIRECTIONS, grid: &Vec<Vec<CELLVAL>>) -> bool {
    let mut nextX = currentX;
    let mut nextY = currentY;

    match direction{
        DIRECTIONS::Up=>nextY -= 1,
        DIRECTIONS::Down=>nextY += 1,
        DIRECTIONS::Left=>nextX -= 1,
        DIRECTIONS::Right=>nextX += 1
    }

    return grid[nextY][nextX] == CELLVAL::Wall;

}

// dimX and dimY should be creater than like 3x3 or something silly.
pub fn createBorders() -> Vec<(usize, usize)> {
    let mut finalBorders: Vec<(usize, usize)> = vec![];
    for x in 0..MAP_DIM_X {
        finalBorders.push((x, 0));
        finalBorders.push((x, MAP_DIM_Y - 1));
    }
    for y in 1..MAP_DIM_Y {
        finalBorders.push((0, y));
        finalBorders.push((MAP_DIM_X - 1, y));
    }

    return finalBorders;
}