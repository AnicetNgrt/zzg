use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn equals(&self, other: &Position, board_width: usize, board_height: usize) -> bool {
        return other.x.rem_euclid(board_width) == self.x.rem_euclid(board_width)
            && other.y.rem_euclid(board_height) == self.y.rem_euclid(board_height);
    }
}
