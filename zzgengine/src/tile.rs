use serde::{Serialize, Deserialize};

use crate::position::Position;

#[derive(Serialize, Deserialize, Clone)]
pub struct Tile {
    pub position: Position
}