use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Rules {
    pub player_count: usize,
    pub min_pawn_count: usize,
    pub initial_pawn_count: usize,
    pub max_pawn_count: usize,
    pub board_width: usize,
    pub board_height: usize,
    pub min_mana_per_turn: i32,
    pub max_mana_per_turn: i32,
}

impl Rules {
    pub fn default() -> Self {
        Self {
            player_count: 2,
            min_pawn_count: 0,
            initial_pawn_count: 3,
            max_pawn_count: 4,
            board_width: 10,
            board_height: 15,
            min_mana_per_turn: 4,
            max_mana_per_turn: 4,
        }
    }
}
