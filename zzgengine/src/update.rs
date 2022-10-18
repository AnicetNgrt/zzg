use dyn_clone::DynClone;

use crate::{state::State};

#[derive(Clone)]
pub struct Update {
    pub done: bool,
    pub kind: Box<dyn UpdateKind>
}

impl Update {
    pub fn new(kind: Box<dyn UpdateKind>) -> Self {
        Self { done: false, kind }
    }

    pub fn execute(&mut self, state: &mut State) {
        self.kind.execute(state);
        self.done = true;
    }

    pub fn undo(&mut self, state: &mut State) {
        self.kind.undo(state);
        self.done = false;
    }
}

pub trait UpdateKind: DynClone {
    fn execute(&self, state: &mut State);
    fn undo(&self, state: &mut State);
}

dyn_clone::clone_trait_object!(UpdateKind);