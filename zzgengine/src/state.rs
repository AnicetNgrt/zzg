use serde::{Deserialize, Serialize};

use crate::{
    pawn::{Pawn, PawnState},
    player::Player,
    tile::Tile, rules::Rules,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct State 
{
    turn: usize,
    players: Vec<Player>,
    pawns: Vec<Pawn>,
    board: Vec<Tile>,
    rules: Rules
}

impl State
{
    pub fn initial(rules: Rules) -> Self {
        State {
            turn: 0,
            players: Vec::new(),
            pawns: Vec::new(),
            board: Vec::new(),
            rules
        }
    }
    
    pub fn is_players_turn(&self, player_id: usize) -> bool {
        player_id == self.turn % self.players.len()
    }

    pub fn is_pawn_existing(&self, pawn_id: usize) -> bool {
        self.pawns.get(pawn_id).is_some()
    }

    pub fn get_pawn(&self, pawn_id: usize) -> &Pawn {
        self.pawns.get(pawn_id).expect(&format!("Internally queried pawn {} does not exist", pawn_id))
    }

    pub fn get_pawn_mut(&mut self, pawn_id: usize) -> &mut Pawn {
        self.pawns.get_mut(pawn_id).expect(&format!("Internally mutably queried pawn {} does not exist", pawn_id))
    }

    pub fn get_tile(&self, tile_id: usize) -> &Tile {
        self.board.get(tile_id).expect(&format!("Internally queried tile {} does not exist", tile_id))
    }

    pub fn is_tile_free(&self, tile_id: usize) -> bool {
        let tile = self.get_tile(tile_id);
        self.pawns.iter().all(|pawn| match pawn.state {
            PawnState::Placed(pawns_tile_id) => {
                let pawns_tile = self.get_tile(pawns_tile_id);
                tile.position.equals(&pawns_tile.position, self.rules.board_width, self.rules.board_height)
            }
            _ => true,
        })
    }

    // pub fn update_pawns(&mut self, pawns: BoxedIter<(usize, Rc<Pawn>)>)
    // {
    //     self.pawns = Rc::new(HashMap::from_iter(pawns));
    // }
}
