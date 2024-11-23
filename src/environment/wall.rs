
use crate::*;
use crate::constants;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Wall {
    pos_x: i32,
    pos_y: i32
}

impl Wall {
    fn new() -> Self {
        Self {
            pos_x: 0,
            pos_y: 0
        }
    }
}

pub fn isWall(currentX: i32, currentY: i32, direction: constants::DIRECTIONS, state: GameState) {
    
    if direction == constants::DIRECTIONS::Up {
        log("asdf");
    }
}