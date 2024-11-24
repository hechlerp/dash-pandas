use crate::*;
use crate::constants::{DIRECTIONS, CELLVAL};
use crate::environment::{isNextStepPosCellAWall};

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PlayerCharacter {
    pub position: (usize, usize),
    pub playerId: String,
    pub assingedCellVal: CELLVAL
}

impl PlayerCharacter {
    pub fn new(player: String) -> Self {
        Self {
            position: (1, 1), // Position in grid
            playerId: player,
            assingedCellVal: CELLVAL::NotAssigned
        }
    }

    pub fn WriteInGrid() {
        // let state = os::server::read!(ServerGameState, "game_state");
        // state.grid[position.0][position.1] = assingedCellVal;
    }

    pub fn RemoveFromGrid() {
        // let state = os::server::read!(ServerGameState, "game_state");
        // state.grid[position.0][position.1] = CELLVAL::Empty;
    }

    pub fn moveInDirection(&mut self, direction: constants::DIRECTIONS, grid: &Vec<Vec<CELLVAL>>) {

        Self::RemoveFromGrid();

        let mut loopCount = 0;
        let mut maxLoopIterations = 100;

        while (loopCount < maxLoopIterations &&
            !(isNextStepPosCellAWall(self.position.0, self.position.1, &direction, &grid))) {

            loopCount += 1;

            if Self::isNextStepPosAnEnemy(self, &direction, &grid) {
                // Attack enemy
            }

            let mut nextPos: (usize, usize);

            match direction {
                DIRECTIONS::Up => nextPos = (self.position.0, self.position.1 - 1),
                DIRECTIONS::Down => nextPos = (self.position.0, self.position.1 + 1),
                DIRECTIONS::Left => nextPos = (self.position.0 - 1, self.position.1),
                DIRECTIONS::Right => nextPos = (self.position.0 + 1, self.position.1)
            }

            Self::move_to(self, nextPos.0, nextPos.1);  
        }

        Self::WriteInGrid();
    }

    pub fn isNextStepPosAnEnemy(&self, direction: &constants::DIRECTIONS, grid: &Vec<Vec<CELLVAL>>) -> bool {
        let mut nextX = self.position.0;
        let mut nextY = self.position.1;
        match direction {
            DIRECTIONS::Up => nextY += 1,
            DIRECTIONS::Down => nextY -= 1,
            DIRECTIONS::Left => nextX -= 1,
            DIRECTIONS::Right => nextX += 1
        }
        
        let isPlayerExistsInNextCell = grid[nextY][nextX] == CELLVAL::P1 || grid[nextY][nextX] == CELLVAL::P2;
        let isPlayerInNextCellSelf = grid[nextY][nextX] == self.assingedCellVal;

        return isPlayerExistsInNextCell && !isPlayerInNextCellSelf; 
    }

    pub fn move_to(&mut self, nextX: usize, nextY: usize) {
        self.position = (nextX, nextY);
    }
}