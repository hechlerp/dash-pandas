use crate::*;
use std::fmt;



#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum DIRECTIONS {
    Up,
    Down,
    Left,
    Right
}


impl fmt::Display for DIRECTIONS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}