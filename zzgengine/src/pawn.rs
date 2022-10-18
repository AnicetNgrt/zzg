use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Pawn {
    pub owner_id: usize,
    pub state: PawnState
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PawnState {
    Unplaced,
    Placed(usize)
}