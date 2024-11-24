use crate::*;
use crate::constants::{DIRECTIONS, CELLVAL};
use crate::environment::{isNextStepPosCellAWall};

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PlayerCharacter {
    pub position: (usize, usize),
    pub playerId: String,
    pub assingedCellVal: CELLVAL,
    pub playerNum: usize
}

impl PlayerCharacter {
    pub fn new(player: String, num: usize) -> Self {
        let mut playerRole: CELLVAL = if num == 0 {CELLVAL::P1} else {CELLVAL::P2};
        let spawn_pos : (usize, usize) = (1, 1);//if num == 0 {(1, 1)} else {(16, 7)};
        Self {
            position: spawn_pos, // Position in grid
            playerId: player,
            assingedCellVal: playerRole,
            playerNum: num
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

            let mut nextPos: (usize, usize);
            match direction {
                DIRECTIONS::Up => nextPos = (self.position.0 - 1, self.position.1),
                DIRECTIONS::Down => nextPos = (self.position.0 + 1, self.position.1 ),
                DIRECTIONS::Left => nextPos = (self.position.0, self.position.1- 1),
                DIRECTIONS::Right => nextPos = (self.position.0, self.position.1 + 1)
            }
            if self.isNextStepPosAnEnemy(nextPos,  &grid) {
                // Attack enemy
            }



            Self::move_to(self, nextPos.0, nextPos.1);  
        }

        Self::WriteInGrid();
    }

    pub fn getMovementSpaceInDir(&mut self, direction: constants::DIRECTIONS, grid: &Vec<Vec<CELLVAL>>) -> ((usize, usize), bool) {
        let mut loopCount = 0;
        let mut maxLoopIterations = 100;
        let mut didEncounterFoe: bool = false;
        let mut currentStep: (usize, usize) = (self.position.0, self.position.1);
        while (loopCount < maxLoopIterations &&
            !(isNextStepPosCellAWall(self.position.0, self.position.1, &direction, &grid))) {

            loopCount += 1;
            
            let mut nextPos: (usize, usize);

            match direction {
                DIRECTIONS::Up => nextPos = (self.position.0, self.position.1 - 1),
                DIRECTIONS::Down => nextPos = (self.position.0, self.position.1 + 1),
                DIRECTIONS::Left => nextPos = (self.position.0 - 1, self.position.1),
                DIRECTIONS::Right => nextPos = (self.position.0 + 1, self.position.1)
            }

            if self.isNextStepPosAnEnemy(nextPos,&grid) {
                didEncounterFoe = true;
                // Attack enemy
            }
            
            currentStep = (nextPos.0, nextPos.1);
            // Self::move_to(self, nextPos.0, nextPos.1);  
        } 
        return (currentStep, didEncounterFoe);

    }

    pub fn isNextStepPosAnEnemy(&self, nextPos: (usize, usize), grid: &Vec<Vec<CELLVAL>>) -> bool {
        let (nextX, nextY) = nextPos;
        
        let isPlayerExistsInNextCell = grid[nextY][nextX] == CELLVAL::P1 || grid[nextY][nextX] == CELLVAL::P2;
        let isPlayerInNextCellSelf = grid[nextY][nextX] == self.assingedCellVal;

        return isPlayerExistsInNextCell && !isPlayerInNextCellSelf; 
    }

    pub fn move_to(&mut self, nextX: usize, nextY: usize) {
        self.position = (nextX, nextY);
    }
}