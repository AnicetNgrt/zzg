use crate::{
    action::{ActionKind, IllegalActionCulprit, IllegalActionReason},
    aliases::{YesOrNo::{self, No, Yes}},
    state::State,
    update::Update, update_kinds::ChangePawnStateUpdate,
};

pub struct PlacePawnActionKind  {
    pub pawn_id: usize,
    pub tile_id: usize,
}

impl ActionKind for PlacePawnActionKind {
    fn is_legal(&self, state: &State, player_id: usize) -> YesOrNo<Vec<IllegalActionCulprit>> {
        let mut illegal_reason = vec![];
        let pawn = state.get_pawn(self.pawn_id);

        if pawn.owner_id != player_id {
            illegal_reason.push(IllegalActionCulprit::from_pawn(
                self.pawn_id,
                IllegalActionReason::PlayerDoesntOwn,
            ));
        }

        if !state.is_tile_free(self.tile_id) {
            illegal_reason.push(IllegalActionCulprit::from_tile(
                self.tile_id,
                IllegalActionReason::TileIsNotFree,
            ));
        }

        if illegal_reason.len() > 0 {
            No(illegal_reason)
        } else {
            Yes
        }
    }

    fn build_updates(&self, state: &State, _player_id: usize) -> Vec<Update> {
        let updates = vec![
            Update::new(Box::new(
                ChangePawnStateUpdate::to_placed(state, self.pawn_id, self.tile_id)    
            ))
        ];
        updates
    }
}
