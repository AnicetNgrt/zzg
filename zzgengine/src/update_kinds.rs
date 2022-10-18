use crate::{
    pawn::{PawnState},
    state::State,
    update::UpdateKind
};

#[derive(Clone)]
pub struct ChangePawnStateUpdate {
    pub pawn_id: usize,
    pub previous_state: PawnState,
    pub next_state: PawnState,
}

impl ChangePawnStateUpdate {
    pub fn to_placed(state: &State, pawn_id: usize, tile_id: usize) -> Self {
        Self {
            pawn_id,
            previous_state: state.get_pawn(pawn_id).state.clone(),
            next_state: PawnState::Placed(tile_id),
        }
    }
}

impl UpdateKind for ChangePawnStateUpdate {
    fn execute(&self, state: &mut State) {
        let mut pawn = state.get_pawn_mut(self.pawn_id);
        pawn.state = self.next_state.clone();
    }

    fn undo(&self, state: &mut State) {
        let mut pawn = state.get_pawn_mut(self.pawn_id);
        pawn.state = self.previous_state.clone();
    }
}
