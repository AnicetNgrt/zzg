use crate::{state::State, aliases::{YesOrNo, Interactible}, update::Update};

pub struct Action {
    pub player_id: usize,
    pub kind: Box<dyn ActionKind>
}

impl Action {
    pub fn is_legal(&self, state: &State) -> YesOrNo<Vec<IllegalActionCulprit>> {
        self.kind.is_legal(state, self.player_id)
    }

    pub fn build_updates(&self, state: &State) -> Vec<Update> {
        self.kind.build_updates(state, self.player_id)
    }
}

pub trait ActionKind {
    fn is_legal(&self, state: &State, player_id: usize) -> YesOrNo<Vec<IllegalActionCulprit>>;
    fn build_updates(&self, state: &State, player_id: usize) -> Vec<Update>;
}

pub enum IllegalActionReason {
    PlayerDoesntOwn,
    TileIsNotFree
}

pub struct IllegalActionCulprit {
    pub interactible: Interactible,
    pub reason: IllegalActionReason
}

impl IllegalActionCulprit {
    pub fn from_pawn(pawn_id: usize, reason: IllegalActionReason) -> Self {
        Self { interactible: Interactible::Pawn(pawn_id), reason }
    }

    pub fn from_tile(tile_id: usize, reason: IllegalActionReason) -> Self {
        Self { interactible: Interactible::Tile(tile_id), reason }
    }
}