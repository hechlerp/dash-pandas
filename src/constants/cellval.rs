use crate::*;



#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum CELLVAL {
    Empty,
    Wall,
    P1,
    P2,
    NotAssigned
}